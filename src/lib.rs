// extern crate alga;
extern crate num_traits as num;
extern crate nalgebra as na;
extern crate rand;

#[cfg(feature = "bin")]
extern crate num_cpus;

#[macro_use]
mod macros;
mod space;
mod math;
mod ray;
mod img;

pub mod material;
pub mod shape;
pub mod primitive;
pub mod light;
pub mod scene;

pub use space::{Point, Color, Vector};
pub use scene::Scene;
pub use img::{Film, Pixel, PixelBuffer};

use std::thread;
use std::ptr::NonNull;

use ray::primary::PrimaryRay;

/// Render the given scene. Returns a Film instance, over you may iterate with
/// the foreach method.
pub fn render(scene: &Scene) -> Film {
    let (width, height) = scene.options.dimensions;
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
    let mut threads: Vec<thread::JoinHandle<_>> = vec![];

    for i in 1..barrel_count {
        // This weird unsafe pointer casting is to allow multiple instances of
        // the constant scene reference and the mutable pixel buffer reference
        // to be used by the tracing code.
        //
        // This allows the various capture_chunk calls to operate on the same
        // block of memory concurrently without having to break it up, rearrange
        // it, and put it back together at the end.
        //
        // It's very important that the threads join the main thread before
        // this function call ends, otherwise very bad things will happen.
        let sendable_scene_ptr = Wrapper(scene as *const Scene);
        let sendable_pixel_ptr = WrapperMut(unsafe { NonNull::new_unchecked(pixel_ptr) });

        let handle = thread::spawn(move || {
            let scene: &Scene = unsafe { &*sendable_scene_ptr.0 };
            let pixels: *mut Pixel = sendable_pixel_ptr.0.as_ptr();
            capture_chunk(i, barrel_count, scene, pixels)
        });

        threads.push(handle)
    }

    // Ensure main thread does processing (see above about unsafe calls)
    let sendable_pixel_ptr = WrapperMut(unsafe { NonNull::new_unchecked(pixel_ptr) });
    capture_chunk(0, barrel_count, scene, sendable_pixel_ptr.0.as_ptr());

    // IMPORTANT: Ensure the threads join before the function returns. Otherwise
    // the Scene reference might disappear and everything will explode.
    for thread in threads { thread.join().unwrap() }
}

/// Capture chunk k of n for the given scene.
/// The pixels pointer is the start of the image buffer.
/// The pointer must allow data access into (width * height) pixels.
fn capture_chunk(k: u8, n: u8, scene: &Scene, pixels: *mut Pixel) {
    let (width, height) = scene.options.dimensions;
    let up = &scene.up;
    let aux = &scene.aux;
    let sample_distance = scene.sample_radius * 2.0;

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

    // Skip over irrelevant chunks
    for offset in ((k as isize)..capacity).step_by(n as usize) {
        let x = (offset % width as isize) as f64;
        let y = (offset / height as isize) as f64;

        // Calculate offsets distances from the view vector
        let hoffset = (x - ((width as f64 - 1.0) * 0.5)) * sample_distance;
        let voffset = ((height as f64 - 1.0) * 0.5 - y) * sample_distance;

        // The direction in which this ray travels
        let d: Vector = scene.view + (voffset * up) + (hoffset * aux);

        let ray = PrimaryRay::new(scene.eye, d);
        let color = ray.cast(scene);

        // This is okay to do assuming the pixel buffer is always the correct
        // size. See the capture method for why this is necessary
        let pixel: Option<&mut Pixel> = unsafe { pixels.offset(offset).as_mut() };

        match pixel {
            Some(pixel) => img::set_pixel_color(pixel as &mut Pixel, &color),
            None => debug_assert!(0 == 1, "Invalid pixel location!")
        }
    }
}

#[cfg(feature = "bin")]
fn get_max_threads() -> u8 { num_cpus::get() as u8 }
#[cfg(not(feature = "bin"))]
fn get_max_threads() -> u8 { 1 }

// Funky Pointer containers to allow sharing pointers between threads
// Need this for the capture function.
struct Wrapper<T>(*const T);
struct WrapperMut<T>(NonNull<T>);
unsafe impl<T> std::marker::Send for Wrapper<T> {}
unsafe impl<T> std::marker::Send for WrapperMut<T> {}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn it_works() {
        assert!(true);
    }
}
