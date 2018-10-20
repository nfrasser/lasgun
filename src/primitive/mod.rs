use crate::{ space::*, ray::Ray,  interaction::SurfaceInteraction };

/// A primitive is a 3D shape placed in the scene. All primitives can intersect
/// with a Ray defined by an origin point and (d)irection vector.
///
/// The returned material reference must have at least the same lifetime as the
/// Scene and the primitive to which it belongs.
pub trait Primitive {
    // Object-level bounds for this primitive
    fn bound(&self) -> Bounds;

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool;
    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray, &mut SurfaceInteraction::none())
    }
}

pub mod geometry;
