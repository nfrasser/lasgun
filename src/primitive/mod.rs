use crate::scene::Scene;
use crate::material::Material;
use crate::shape::Intersection;
use crate::ray::Ray;

/// A primitive is a 3D shape placed in the scene. All primitives can intersect
/// with a Ray defined by an origin point and (d)irection vector.
///
/// The returned material reference must have at least the same lifetime as the
/// Scene and the primitive to which it belongs.
pub trait Primitive {
    fn material<'a>(&'a self, scene: &'a Scene) -> &'a dyn Material;
    fn intersect(&self, ray: &Ray) -> (Intersection, &dyn Primitive);
}

pub mod geometry;
