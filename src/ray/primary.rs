use rand::prelude::*;
use rand::distributions::{StandardNormal};

use std::f64;
use na::Unit;
use space::{ Vector, Point, Color, Direction };
use scene::Scene;
use ray::Ray;

/**
The initial ray that gets cast from the camera to scene.
The resulting colour form the `cast` function will be a pixel in the resulting image.
*/
pub struct PrimaryRay {
    pub e: Point, // eye/camera position in space
    pub d: Vector // direction from eye into which the ray is cast
}

impl PrimaryRay {
    pub fn new(e: Point, d: Vector) -> PrimaryRay {
        PrimaryRay { e, d }
    }

    /**
    Get a direction ray that's within a small distance from the original point of
    intersection on the focal plane of the given scene.
    */
    fn supersample(&self, scene: &Scene) -> Direction {
        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
        let upfactor = scene.sample_radius * rng.sample(StandardNormal);
        let auxfactor = scene.sample_radius * rng.sample(StandardNormal);

        // New point at which the ray intersects the focal plane given this direction
        let d = self.d + (upfactor * scene.up) + (auxfactor * scene.aux);
        Direction::new(d)
    }
}

impl Ray for PrimaryRay {
    fn cast(&self, scene: &Scene) -> Color {
        let samples = scene.options.supersampling.max(1);
        let mut color = Color::zeros();

        for i in 0..samples {

            let direction = if i == 0 { Direction::new(self.d) } else { self.supersample(scene) };
            let (intersection, primitive) = scene.intersect(&self.e, &direction);

            // Get the material the interecting primitive is made of
            let material = primitive.material();

            // The vector spanning from the eye to the point of intersection
            // eye + direction = point of intersection
            let direction: Vector = intersection.t * direction.d;
            let normal = Unit::new_normalize(intersection.normal);

            // Add a small fraction of the normal to avoid speckling due to floating point errors
            // (the calculated point ends up inside the geometric primitive).
            let qpoint = self.e + direction + (f64::EPSILON * 32.0) * normal.as_ref();

            // Query the material for the color at the given point
            color += material.color(&qpoint, &self.e, &normal, scene)
        }

        color * scene.supersample_power
    }
}
