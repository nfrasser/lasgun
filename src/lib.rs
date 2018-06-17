extern crate nalgebra as na;

use std::f64;

mod space;
mod math;
mod img;

pub mod material;
pub mod shape;
pub mod primitive;
pub mod light;
pub mod scene;
mod ray;


pub use space::{Point, Color, Vector};
pub use scene::Scene;
pub use img::{Image, ImageBuffer};

use ray::Ray;
use ray::primary::PrimaryRay;

pub fn render_to(image: &mut Image, scene: &Scene) {
    let (width, height) = scene.dimensions;

    // The Auxilary Vector is normal to the view and up vectors
    let aux = scene.view.cross(&scene.up);
    let up = aux.cross(&scene.view).normalize();
    let aux = aux.normalize();

    // First point of the target plane will be at this distance from the eye
    let d = space::len(&scene.view);

    // Half the height of the point grid in model coordinates
    let ymax = d * f64::tan((1.0/360.0) * scene.fov * f64::consts::PI);

    let pixelwidth = 2.0 * ymax / height as f64;

    for j in 0..height {

        let voffset = ((height as f64 - 1.0) * 0.5 - j as f64) * pixelwidth;

        // A point on the jth row on the same plane as the up and direction vectors
        let vraypoint: Point = scene.eye + (voffset * up) + scene.view;

        for i in 0..width {
            let hoffset = (i as f64 - ((height as f64 - 1.0) * 0.5)) * pixelwidth;

            // The point at which the ray intersects
            let d: Vector = vraypoint + (hoffset * aux) - scene.eye;
            let ray = PrimaryRay::new(scene.eye, d);
            let color = ray.cast(&scene);
            image.set_pixel_color(i, j, &color);
        }
    }
}

pub fn render(scene: &Scene) -> ImageBuffer {
    let (width, height) = scene.dimensions;
    let mut image = ImageBuffer::new(width, height);
    render_to(&mut image, scene);
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
