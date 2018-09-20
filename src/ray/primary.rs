use std::f64;

use space::*;
use scene::Scene;
use primitive::Primitive;

use super::Ray;

/**
The initial ray that gets cast from the camera to scene.
The resulting colour form the `cast` function will be a pixel in the resulting image.
*/
#[derive(Debug, Copy, Clone)]
pub struct PrimaryRay {
    pub origin: Point,
    pub d: Vector // not normalized
}

impl PrimaryRay {
    /**
    Create a new primary ray from an origin point and a vector that spans from the origin
    to the focal plane
    */
    pub fn new(origin: Point, d: Vector) -> PrimaryRay {
        PrimaryRay { origin, d }
    }

    pub fn cast(&self, scene: &Scene) -> Color {
        let dim = scene.supersampling.dim as i32;
        let mut color = Color::zeros();

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

            let (intersection, primitive) = scene.contents.intersect(&ray);

            // Get the material the interecting primitive is made of
            let material = primitive.material(scene);

            // The vector spanning from the eye to the point of intersection
            // eye + direction = point of intersection
            let direction: Vector = intersection.t * ray.d;
            let normal: &Normal = &intersection.normal;

            // Add a small fraction of the normal to avoid speckling due to floating point errors
            // (the calculated point ends up inside the geometric primitive).
            let qpoint = ray.origin + direction + (f64::EPSILON * 32.0) * normal.as_ref();

            // Query the material for the color at the given point
            color += material.color(&qpoint, &ray.origin, normal, scene)
        }

        color * scene.supersampling.power
    }
}
