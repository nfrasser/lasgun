use crate::{ space::*, ray::Ray,  interaction::SurfaceInteraction };

/// A primitive is a 3D shape placed in the scene. All primitives can intersect
/// with a Ray defined by an origin point and (d)irection vector.
///
/// The returned material reference must have at least the same lifetime as the
/// Scene and the primitive to which it belongs.
pub trait Primitive {
    fn object_bound(&self) -> Bounds;
    fn world_bound(&self, object_to_world: &Transformation) -> Bounds {
        object_to_world.transform_bounds(self.object_bound())
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray, &mut SurfaceInteraction::none())
    }
}

pub mod geometry;
