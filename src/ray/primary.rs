use std::f64;
// use rand::{ prelude::*, distributions::StandardNormal };

use space::*;
use scene::Scene;

use super::Ray;

/**
The initial ray that gets cast from the camera to scene.
The resulting colour form the `cast` function will be a pixel in the resulting image.
*/
#[derive(Debug, Clone)]
pub struct PrimaryRay {
    // Pixel grid index coordinates
    pub x: u16,
    pub y: u16,
    pub origin: Point,
    pub d: Vector, // not normalized
    pub color: Color,
}

impl PrimaryRay {
    /**
    Create a new primary ray from an origin point and a vector that spans from the origin
    to the focal plane
    */
    pub fn new(x: u16, y: u16, origin: Point, d: Vector) -> PrimaryRay {
        PrimaryRay { x, y, origin, d, color: Color::zeros() }
    }

    pub fn cast(&mut self, scene: &Scene) {
        let dim = scene.supersampling.dim as i32;

        for i in 0..scene.supersampling.count as i32 {
            // Calculate offset from the origin as factors of the supersampling radius
            // x = {0} => {0}
            // x = {0, 1} => {-1, 1}
            // x = {0, 1, 2} => {-2, 0, 2}
            // x = {0, 1, 2, 3} => {-3, 1, 1, 3}
            let xoffset = i % dim * 2 - dim + 1;
            let yoffset = i / dim * 2 - dim + 1;
            let auxoffset = scene.supersampling.radius * xoffset as f64;
            let upoffset = scene.supersampling.radius * yoffset as f64;

            // New point at which the ray intersects the focal plane given this direction
            let d = self.d + (upoffset * scene.up) + (auxoffset * scene.aux);
            let ray = Ray::new(self.origin, d);

            let (intersection, primitive) = scene.intersect(&ray);

            // Get the material the interecting primitive is made of
            let material = primitive.material();

            // The vector spanning from the eye to the point of intersection
            // eye + direction = point of intersection
            let direction: Vector = intersection.t * ray.d;
            let normal: &Normal = &intersection.normal;

            // Add a small fraction of the normal to avoid speckling due to floating point errors
            // (the calculated point ends up inside the geometric primitive).
            let qpoint = ray.origin + direction + (f64::EPSILON * 32.0) * normal.as_ref();

            // Query the material for the color at the given point
            self.color += material.color(&qpoint, &ray.origin, normal, scene)

        }

        self.color *= scene.supersampling.power
    }


}
