use ::space::{ Point, Vector };
use ::shape::{Shape, Intersection};
use ::material::Material;

use primitive::Primitive;

/**
    A primitive representing a geometric shape such as a sphere or cube.
    The intersection is computed mathematically
*/
pub struct Geometry {
    pub shape: Box<Shape>,
    pub material: Box<Material>
}

impl Primitive for Geometry {
    fn material(&self) -> &Material {
        &*self.material
    }

    fn intersect(&self, e: &Point, d: &Vector) -> (Intersection, &Primitive) {
        (self.shape.intersect(e, d), self)
    }
}
