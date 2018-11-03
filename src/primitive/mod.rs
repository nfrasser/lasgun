use crate::{ space::*, ray::Ray,  interaction::SurfaceInteraction };
use std::marker::Sync;

/// A primitive is a 3D shape placed in the scene. All primitives can intersect
/// with a Ray defined by an origin point and (d)irection vector.
///
/// The returned material reference must have at least the same lifetime as the
/// Scene and the primitive to which it belongs.
pub trait Primitive: Sync {
    // Object-level bounds for this primitive
    fn bound(&self) -> Bounds;

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> Option<&dyn Primitive>;
    fn intersects(&self, ray: &Ray) -> bool {
        if let None = self.intersect(ray, &mut SurfaceInteraction::default()) {
            false
        } else {
            true
        }
    }
}

pub mod geometry;
