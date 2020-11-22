#[cfg(feature = "bin")]
use num_cpus;

#[macro_use]
extern crate bitflags;

#[macro_use]
pub(crate) mod macros;
pub(crate) mod core;
pub(crate) mod camera;
pub(crate) mod img;
pub(crate) mod film;
pub(crate) mod space;
pub(crate) mod interaction;
pub(crate) mod material;
pub(crate) mod shape;
pub(crate) mod primitive;
pub(crate) mod light;
mod accelerators;
mod integrate;

pub mod scene;

#[cfg(feature = "bin")]
pub mod output;

use std::thread;
use std::ptr::NonNull;

pub use crate::scene::Scene;
pub use crate::camera::Camera;
pub use crate::img::{Pixel, PixelBuffer, Img};
pub use crate::film::Film;
pub use crate::primitive::Primitive;
pub use crate::material::Material;

/// A 16×16 portion of pixels taken from a film, arranged in row-major order.
/// Used for streaming render results. NOT a slice of `Film::data`.
///
/// 16 * 16 pixels = 256 pixels = 4 * 256 bytes = 1024 bytes
pub type FilmDataHunk = [u8; 1024];

/// An acceleration structure to reduce the number of ray-object intersection
/// tests. Call the associated `from` method with a scene reference to get back
/// a new primitive to be used for ray intersection.
///
/// Internally implemented as a Bounding-Volume Hierarchy
pub type Accel<'s> = self::accelerators::bvh::BVHAccel<'s>;

/// Render the given scene. Returns a Film instance, over you may iterate with
/// the foreach method.
pub fn render(scene: &Scene, resolution: (u32, u32)) -> Film {
    let mut film = Film::new(resolution.0, resolution.1);
    capture(scene, &mut film);
    film
}

/// Record an image of the scene on the given film. The film must have at least
/// (scene.width * scene.height) pixels reserved in the Film
/// data field.
pub fn capture(scene: &Scene, film: &mut Film) {

    // Get number of threads to use. Uses one by default
    let barrel_count = if scene.threads == 0 {
        get_max_threads()
    } else {
        scene.threads
    };

    let root = Accel::from(scene);
    let mut threads = Vec::with_capacity(barrel_count - 1);

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
        let sendable_film_ptr = UnsafeThreadWrapperMut(NonNull::new(film as *mut Film).unwrap());
        let sendable_root_ptr = UnsafeThreadWrapper(unsafe {
            // I am so, so, so sorry, I need this to get sendable_root_ptr
            // across the thread boundary. I promise I super-quadruple-checked
            // that this is safe.
            std::mem::transmute::<&Accel<'_>, &Accel<'static>>(&root)
        } as *const Accel);

        let handle = thread::spawn(move || {
            let root: &Accel = unsafe { &*sendable_root_ptr.0 };
            let film: &mut Film = unsafe { &mut *sendable_film_ptr.0.as_ptr() };
            capture_subset(i, barrel_count, root, film)
        });

        threads.push(handle)
    }

    // Ensure main thread does processing
    capture_subset(0, barrel_count, &root, film);

    // IMPORTANT: Ensure the threads join before the function returns. Otherwise
    // the Scene reference might disappear and everything will explode.
    for thread in threads { thread.join().unwrap() }
}

/// Get a 16×16 view into the film for the scene starting at coordinates
/// startx/starty. Puts the result in the given film chunk.
// FIXME: Restore this, or do some kind of checkpoint tracing
/*
pub fn capture_hunk(offset: [u32; 2], resolution: [u32; 2], root: &Accel, hunk: &mut FilmDataHunk) {
    let scene = root.scene;
    let (width, height) = (resolution[0], resolution[1]);
    let (startx, starty) = (offset[0], offset[1]);
    debug_assert!(startx < width && starty < height);

    let samples = scene.camera.allocate_samples();
    let weight = 1. / samples.len() as f64;

    for (i, pixel) in hunk.chunks_mut(4).enumerate() { // Iterates 256 times
        let i = i as u32;
        let x = startx + i % 16;
        let y = starty + i / 16;

        // Don't bother rendering pixels outside the frame
        if x >= width || x >= height { continue };

        scene.camera.sample(x, y, )
        let color = ray.cast(root);

        let pixel: &mut [RgbaPixel] = unsafe { std::mem::transmute(pixel) };
        img::set_pixel_color(&mut pixel[0], &color)
    }
}
*/

/// Capture subset k of n for the given scene. That is, every kth pixel in the
/// pixel buffer, arranged in row-major order. The pixel pointer is the start of
/// the image buffer. The pointer must allow data access into
/// (scene.width * scene.height) pixels.
pub fn capture_subset(k: usize, n: usize, root: &Accel, img: &mut impl Img) {
    let scene = root.scene;
    let (width, height) = (img.w() as usize, img.h() as usize);

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
    let area = width * height; // total image area
    let mut samples = scene.camera.allocate_samples();
    let weight = 1. / samples.len() as f64;

    // Skip over chunks that other threads are processing/ Assuming
    // capture_subset is never called concurrently with the same k and n values,
    // this will never cause contention/race conditions.
    for offset in ((k as usize)..area).step_by(n as usize) {
        debug_assert!(offset < area);
        let x = (offset % width) as u32;
        let y = (offset / width) as u32;
        debug_assert!(x < img.w());
        debug_assert!(y < img.h());
        scene.camera.sample(x, y, img, &mut samples);
        let color = integrate::integrate(root, &samples, weight);
        img.set(x, y, &color.into())
    }
}

#[cfg(feature = "bin")]
fn get_max_threads() -> usize { num_cpus::get() }
#[cfg(not(feature = "bin"))]
fn get_max_threads() -> usize { 1 }

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
