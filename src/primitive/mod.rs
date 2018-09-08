use scene::Scene;
use material::Material;
use shape::Intersection;
use ray::Ray;

/// A primitive is a 3D shape placed in the scene. All primitives can intersect
/// with a Ray defined by an origin point and (d)irection vector.
///
/// The returned material reference must have at least the same lifetime as the
/// Scene and the primitive to which it belongs.
pub trait Primitive {
    fn material<'a>(&'a self, scene: &'a Scene) -> &'a Material;
    fn intersect(&self, ray: &Ray) -> (Intersection, &Primitive);
}

pub mod aggregate;
pub mod geometry;
