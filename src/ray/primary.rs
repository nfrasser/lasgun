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
    pub d: Direction, // direction from eye into which the ray is cast
}

impl PrimaryRay {
    pub fn new(e: Point, d: Vector) -> PrimaryRay {
        PrimaryRay { e, d: Direction::new(d) }
    }
}

impl Ray for PrimaryRay {
    fn cast(&self, scene: &Scene) -> Color {
        let (intersection, primitive) = scene.intersect(&self.e, &self.d);

        // Get the material the interecting primitive is made of
        let material = primitive.material();

        // The vector spanning from the eye to the point of intersection
        // eye + direction = point of intersection
        let direction: Vector = intersection.t * self.d.d;
        let normal = Unit::new_normalize(intersection.normal);

        // Add a small fraction of the normal to avoid specling due to floating point errors
        // (the calculated point ends up inside the geometric primitive).
        let qpoint = self.e + direction + (f64::EPSILON * 32.0) * normal.as_ref();

        // Query the material for the color at the given point
        material.color(&qpoint, &self.e, &normal, scene)
    }
}
