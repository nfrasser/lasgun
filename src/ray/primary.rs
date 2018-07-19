use std::f64;
use rand::{ prelude::*, distributions::StandardNormal };
use na::Unit;

use space::*;
use scene::Scene;

use super::Ray;

/**
The initial ray that gets cast from the camera to scene.
The resulting colour form the `cast` function will be a pixel in the resulting image.
*/
pub struct PrimaryRay {
    origin: Point,
    d: Vector, // not normalized
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
        let samples = scene.options.supersampling.max(1);
        let mut color = Color::zeros();

        for i in 0..samples {

            // Sample a ray that's just slighly off from the original
            let ray = if i == 0 {
                Ray::new(self.origin, self.d)
            } else {
                self.supersample(scene)
            };

            let (intersection, primitive) = scene.intersect(&ray);

            // Get the material the interecting primitive is made of
            let material = primitive.material();

            // The vector spanning from the eye to the point of intersection
            // eye + direction = point of intersection
            let direction: Vector = intersection.t * ray.d;
            let normal = Unit::new_normalize(intersection.normal);

            // Add a small fraction of the normal to avoid speckling due to floating point errors
            // (the calculated point ends up inside the geometric primitive).
            let qpoint = ray.origin + direction + (f64::EPSILON * 32.0) * normal.as_ref();

            // Query the material for the color at the given point
            color += material.color(&qpoint, &ray.origin, &normal, scene)
        }

        color * scene.supersample_power
    }

    /**
    Get a direction ray that's within a small distance from the original point of
    intersection on the focal plane of the given scene.
    Returns a new origin and directoin vector
    */
    fn supersample(&self, scene: &Scene) -> Ray {
        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

        // Angle between 0 and π at which the new ray deviates
        let angle = scene.random_angle(&mut rng);

        // Distance from the original point of intersection
        let distance = rng.sample(StandardNormal);

        let upoffset = scene.sample_radius * angle.sin() * distance;
        let auxoffset = scene.sample_radius * angle.cos() * distance;

        // New point at which the ray intersects the focal plane given this direction
        let d = self.d + (upoffset * scene.up) + (auxoffset * scene.aux);
        Ray::new(self.origin, d)
    }
}
