use crate::space::*;
use crate::ray::Ray;
use crate::interaction::SurfaceInteraction;

/// A gemetric shape that lives in 3D space. Must implement a way to intersect
/// with a given ray
pub trait Shape {
    /// Bounding box in object coordinates
    fn object_bound(&self) -> Bounds;
    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool;

    fn world_bound(&self, object_to_world: &Transformation) -> Bounds {
        object_to_world.transform_bounds(self.object_bound())
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray, &mut SurfaceInteraction::none())
    }
}

pub mod cuboid;
pub mod sphere;
pub mod mesh;
pub mod triangle;
