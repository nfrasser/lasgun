#[cfg(feature = "bin")]
use num_cpus;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod math;
pub(crate) mod ray;
pub(crate) mod img;
pub(crate) mod space;
pub(crate) mod interaction;
pub(crate) mod material;
pub(crate) mod shape;
pub(crate) mod primitive;
pub(crate) mod light;
pub(crate) mod accelerators;

pub mod scene;

#[cfg(feature = "bin")]
pub mod output;

use std::thread;
use std::ptr::NonNull;

pub use crate::scene::Scene;
pub use crate::img::{Film, Pixel, PixelBuffer};
use crate::ray::primary::PrimaryRay;
use crate::primitive::Primitive;

/// A 16×16 portion of pixels taken from a film, arranged in row-major order.
/// Used for streaming render results. NOT a slice of `Film::data`.
///
/// 16 * 16 pixels = 256 pixels = 4 * 256 bytes = 1024 bytes
pub type FilmDataHunk = [u8; 1024];

/// An acceleration structure tied to a give scene
/// Internally implemented as a Bounding-Volume Hierarchy
pub type Accel<'s> = self::accelerators::bvh::BVHAccel<'s>;

/// Render the given scene. Returns a Film instance, over you may iterate with
/// the foreach method.
pub fn render(scene: &Scene) -> Film {
    let (width, height) = (scene.options.width, scene.options.height);
    let mut film = Film::new(width, height);
    capture(scene, &mut film);
    film
}

/// Record an image of the scene on the given film. The film must have at least
/// (scene.options.width * scene.options.height) pixels reserved in the Film
/// data field.
pub fn capture(scene: &Scene, film: &mut Film) {

    // Get number of threads to use. Uses one by default
    let barrel_count = if scene.options.threads == 0 {
        get_max_threads()
    } else {
        scene.options.threads
    };

    let pixel_ptr = film.data.raw_pixels_mut();

    let root = Accel::from(scene);
    let mut threads: Vec<thread::JoinHandle<_>> = vec![];

    for i in 1..barrel_count {

        // This weird unsafe pointer casting is to allow multiple instances of
        // the constant scene reference and the mutable pixel buffer reference
        // to be used by the tracing code.
        //
        // This allows the various capture_subset calls to operate on the same
        // block of memory concurrently without having to break it up, rearrange
        // it, and put it back together at the end.
        //
        // It's very important that the threads join the main thread before
        // this function call ends, otherwise very bad things will happen.
        //
        // TODO: Pls. make this less terrifying
        let sendable_scene_ptr = UnsafeThreadWrapper(scene as *const Scene);
        let sendable_root_ptr = UnsafeThreadWrapper(unsafe {
            // I am so, so, so sorry, I need this to get sendable_root_ptr
            // across the thread boundary. I promise I super-quadruple-checked
            // that this is safe.
            std::mem::transmute::<&Accel<'_>, &Accel<'static>>(&root)
        } as *const Accel);
        let sendable_pixel_ptr = UnsafeThreadWrapperMut(NonNull::new(pixel_ptr).unwrap());
        let handle = thread::spawn(move || {
            let scene: &Scene = unsafe { &*sendable_scene_ptr.0 };
            let root: &Accel = unsafe { &*sendable_root_ptr.0 };
            let pixels: *mut Pixel = sendable_pixel_ptr.0.as_ptr();
            capture_subset(i, barrel_count, scene, root, pixels)
        });

        threads.push(handle)
    }

    // Ensure main thread does processing
    capture_subset(0, barrel_count, scene, &root, pixel_ptr);

    // IMPORTANT: Ensure the threads join before the function returns. Otherwise
    // the Scene reference might disappear and everything will explode.
    for thread in threads { thread.join().unwrap() }
}

/// Get a 16×16 view into the film for the scene starting at coordinates
/// startx/starty. Puts the result in the given film chunk.
pub fn capture_hunk(startx: u16, starty: u16, scene: &Scene, root: &impl Primitive, hunk: &mut FilmDataHunk) {
    let (width, height) = (scene.options.width, scene.options.height);
    debug_assert!(startx < width && starty < height);

    let up = scene.up;
    let aux = scene.aux;
    let sample_distance = scene.pixel_radius * 2.0;

    for (i, pixel) in hunk.chunks_mut(4).enumerate() { // Iterates 256 times
        let x = (startx as usize + i % 16) as u16;
        let y = (starty as usize + i / 16) as u16;

        // Don't bother rendering pixels outside the frame
        if x >= width || x >= height { continue };

        // Calculate offsets distances from the view vector
        let hoffset = (x as f64 - ((width as f64 - 1.0) * 0.5)) * sample_distance;
        let voffset = ((height as f64 - 1.0) * 0.5 - y as f64) * sample_distance;

        // The direction in which this ray travels
        let d = scene.view + (voffset * up) + (hoffset * aux);

        let ray = PrimaryRay::new(scene.eye, d);
        let color = ray.cast(scene, root);

        let pixel: &mut [Pixel] = unsafe { std::mem::transmute(pixel) };
        img::set_pixel_color(&mut pixel[0], &color)
    }
}

/// Capture subset k of n for the given scene. That is, every kth pixel in the
/// pixel buffer, arranged in row-major order. The pixel pointer is the start of
/// the image buffer. The pointer must allow data access into
/// (scene.width * scene.height) pixels.
fn capture_subset(k: u8, n: u8, scene: &Scene, root: &impl Primitive, pixels: *mut Pixel) {
    let (width, height) = (scene.options.width, scene.options.height);
    let up = scene.up;
    let aux = scene.aux;
    let sample_distance = scene.pixel_radius * 2.0;

    // Render Concurrency Overview
    //
    // Each primary ray is responsible for colouring in a single pixel of the
    // final image.
    //
    // Suppose each pixel in the 4x2 image is labelled numerically as so
    //
    //  [0][1][2][3]
    //  [4][5][6][7]
    //
    // With one thread, the pixels are rendered in order
    // With two threads, these subsets are rendered in parallel
    //  [0][2][4][6]
    //  [1][3][5][7]
    // With three threads
    //  [0][3][6]
    //  [1][4][7]
    //  [2][5]
    // With four threads
    //  [0][4]
    //  [1][5]
    //  [2][6]
    //  [3][7]
    // And so on
    //
    // This pattern guarantees the best possible resource usage for most images
    // (as opposed to splitting the pixel buffer into chunks - some chunks will
    // end up touching more primitives than others!)

    // Calculate the chunk size such that we can yield n chunks,
    // where n is the number of threads
    let capacity = width as isize * height as isize; // total image capacity

    // Skip over chunks that other threads are processing/ Assuming
    // capture_subset is never called concurrently with the same k and n values,
    // this will never cause contention/race conditions.
    for offset in ((k as isize)..capacity).step_by(n as usize) {
        let x = (offset % width as isize) as f64;
        let y = (offset / height as isize) as f64;

        // Calculate offsets distances from the view vector
        let hoffset = (x - ((width as f64 - 1.0) * 0.5)) * sample_distance;
        let voffset = ((height as f64 - 1.0) * 0.5 - y) * sample_distance;

        // The direction in which this ray travels
        let d = scene.view + (voffset * up) + (hoffset * aux);

        let ray = PrimaryRay::new(scene.eye, d);
        let color = ray.cast(scene, root);

        // This is okay to do assuming the pixel buffer is always the correct
        // size. See the capture method for why this is necessary
        let pixel: &mut Pixel = unsafe { pixels.offset(offset).as_mut().unwrap() };
        img::set_pixel_color(pixel as &mut Pixel, &color)
    }
}


#[cfg(feature = "bin")]
fn get_max_threads() -> u8 { num_cpus::get() as u8 }
#[cfg(not(feature = "bin"))]
fn get_max_threads() -> u8 { 1 }

// Funky Pointer containers to allow sharing pointers between threads
// Need this for the capture function.
#[derive(Copy, Clone)] struct UnsafeThreadWrapper<T>(*const T);
#[derive(Copy, Clone)] struct UnsafeThreadWrapperMut<T>(NonNull<T>);
unsafe impl<T> std::marker::Send for UnsafeThreadWrapper<T> {}
unsafe impl<T> std::marker::Send for UnsafeThreadWrapperMut<T> {}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn it_works() {
        assert!(true);
    }
}
