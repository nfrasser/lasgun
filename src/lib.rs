extern crate nalgebra as na;
extern crate rand;

use std::f64;

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
pub use img::{Film, ImageBuffer};

use ray::primary::PrimaryRay;

/**
Record an image of the scene on the given film
*/
pub fn capture(scene: &Scene, film: &mut Film) {
    let (width, height) = scene.options.dimensions;
    let up = &scene.up;
    let aux = &scene.aux;
    let sample_distance = scene.sample_radius * 2.0;

    for j in 0..height {

        let voffset = ((height as f64 - 1.0) * 0.5 - j as f64) * sample_distance;

        // A point on the jth row on the same plane as the up and direction vectors
        let vraypoint: Point = scene.eye + (voffset * up) + scene.view;

        for i in 0..width {
            let hoffset = (i as f64 - ((width as f64 - 1.0) * 0.5)) * sample_distance;

            // The point at which the ray intersects
            let d: Vector = vraypoint + (hoffset * aux) - scene.eye;
            let ray = PrimaryRay::new(scene.eye, d);
            let color = ray.cast(&scene);
            film.set_pixel_color(i, j, &color);
        }
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
