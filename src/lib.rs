// extern crate alga;
extern crate num_traits as num;
extern crate nalgebra as na;
extern crate rand;

#[cfg(feature = "bin")]
extern crate num_cpus;
#[cfg(feature = "bin")]
extern crate crossbeam_utils;

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

use ray::primary::PrimaryRay;

#[cfg(feature = "bin")]
use std::ptr::NonNull;

#[cfg(feature = "bin")]
use crossbeam_utils::thread;

/// Record an image of the scene on the given film
#[cfg(feature = "bin")]
pub fn capture(scene: &Scene, film: &mut Film) {
    let pixel_ptr = film.data.raw_pixels_mut();
    let barrel_count = if scene.options.threads == 0 {
        num_cpus::get() as u8
    } else {
        scene.options.threads
    };

    thread::scope(|scope| {
        for i in 1..barrel_count {
            // All of this funky unsafe code is to allow concurrent access to the
            // constant scene pointer and pixel buffer without requiring mutex primitives
            let sendable_pixel_ptr = Wrapper(unsafe { NonNull::new_unchecked(pixel_ptr) });
            scope.spawn(move || {
                capture_chunk(i, barrel_count, scene, sendable_pixel_ptr.0.as_ptr())
            });
        }

        // Ensure main thread does processing
        let sendable_pixel_ptr = Wrapper(unsafe { NonNull::new_unchecked(pixel_ptr) });
        capture_chunk(0, barrel_count, scene, sendable_pixel_ptr.0.as_ptr())
    })
}

/// Capture chunk k of n for the given scene
/// The pixels pointer is the start of the image buffer
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
        let x: u16 = (offset % width as isize) as u16;
        let y: u16 = (offset / height as isize) as u16;

        let hoffset = (x as f64 - ((width as f64 - 1.0) * 0.5)) * sample_distance;
        let voffset = ((height as f64 - 1.0) * 0.5 - y as f64) * sample_distance;

        // A point on the yth row on the same plane as the up and direction vectors
        let vraypoint: Point = scene.eye + (voffset * up) + scene.view;

        // The point at which the ray intersects
        let d: Vector = vraypoint + (hoffset * aux) - scene.eye;

        let ray = PrimaryRay::new(scene.eye, d);
        let color = ray.cast(scene);
        let pixel: &mut Pixel = unsafe { pixels.offset(offset).as_mut().unwrap() };

        img::set_pixel_color(pixel as &mut Pixel, &color)
    }
}

// Funky Pointer containers to allow sharing mutable pointers between threads
#[cfg(feature = "bin")]
struct Wrapper<T>(NonNull<T>);

#[cfg(feature = "bin")]
unsafe impl<T> std::marker::Send for Wrapper<T> {}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn it_works() {
        assert!(true);
    }
}
