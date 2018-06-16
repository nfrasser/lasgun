use space::{ Point, Vector };
use material::Material;
use shape::Intersection;

// A primitive is a 3D shape placed in the scene.
// All primitives can intersect with a Ray defined by an (E)ye point and (d)irection vector
pub trait Primitive {
    fn material(&self) -> &Material;
    fn intersect(&self, e: &Point, d: &Vector) -> (Intersection, &Primitive);
}

pub mod aggregate;
pub mod geometry;
