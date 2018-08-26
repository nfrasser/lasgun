// extern crate alga;
extern crate num_traits as num;
extern crate nalgebra as na;
extern crate rand;
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
pub use img::{Film, ImageBuffer, Pixel};

use std::thread;
use ray::primary::PrimaryRay;

/**
Record an image of the scene on the given film
*/
pub fn capture(scene: &Scene, film: &mut Film) {
    let (width, height) = scene.options.dimensions;
    let up = &scene.up;
    let aux = &scene.aux;
    let sample_distance = scene.sample_radius * 2.0;

    // Render Concurrency Overview:
    //
    // Each primary ray is responsible for colouring in a single pixel of the
    // final image. Each ray contains a mutable reference to the pixel it
    // colours.
    //
    // The following code generates a slice of primary arrays that are arranged
    // such that chunks from the slice may easily be passed to a thread of
    // parallelizing the render.
    //
    // Suppose each pixel in the 4x2 image is labelled numerically as so
    //
    //  [0][1][2][3]
    //  [4][5][6][7]
    //
    // With one thread, the pixel arrangement doesn't change
    // With two threads
    //  [0][2][4][6] [1][3][5][7]
    // With three threads
    //  [0][3][6] [1][4][7] [2][5]
    // With four threads
    //  [0][4] [1][5] [2][6] [3][7]
    //
    // The spaces deliniate "chunks" of pixels that will be processed by one
    // of the threads.

    // Get number of threads, defaulting to what's allowed by the system
    let barrel_count = if scene.options.concurrency == 0 {
        num_cpus::get()
    } else {
        scene.options.concurrency as usize
    };

    // Calculate the chunk size such that we can yield n chunks,
    // where n is the number of threads
    let capacity = (width * height) as usize; // total ray capacity

    // Capacity per barrel
    let mag_size = capacity / barrel_count + (capacity % barrel_count).min(1);

    // Allocate and initialize the rays
    let mut ammo: Vec<PrimaryRay> = Vec::with_capacity(capacity);
    ammo.resize(capacity, PrimaryRay::new(0, 0, scene.eye, Vector::zeros()));

    // Build up the rays
    for j in 0..height {

        let voffset = ((height as f64 - 1.0) * 0.5 - j as f64) * sample_distance;

        // A point on the jth row on the same plane as the up and direction vectors
        let vraypoint: Point = scene.eye + (voffset * up) + scene.view;

        for i in 0..width {
            let hoffset = (i as f64 - ((width as f64 - 1.0) * 0.5)) * sample_distance;

            // The point at which the ray intersects
            let d: Vector = vraypoint + (hoffset * aux) - scene.eye;

            // Calculate the position within the ammo vector
            let idx = (width * j + i) as usize; // pixel index/label
            let pos = (idx % barrel_count) * mag_size + (idx % mag_size);

            // Update the ray with the correct direction position information
            let mut ray = &mut ammo[pos];
            ray.x = i; ray.y = j; ray.d = d;
        }
    }

    let (first_mag, rest_mags) = ammo.as_mut_slice().split_at_mut(mag_size);
    for mag in rest_mags.chunks_mut(mag_size) {
        thread::spawn(move || for ray in mag.iter_mut() { ray.cast(scene) });
    }
}

/**
Render a scene to the provided ImageBuffer structure
*/
pub fn render(scene: &Scene) -> ImageBuffer {
    let (width, height) = scene.options.dimensions;
    let mut image = ImageBuffer::new(width, height);
    capture(scene, &mut image);
    image
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert!(true);
    }
}
