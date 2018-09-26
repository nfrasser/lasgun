
use crate::ray::Ray;
use crate::scene::{Scene, MaterialRef};
use crate::shape::{
    Shape, Intersection,
};
use crate::material::Material;
use super::Primitive;

/// A primitive representing a geometric shape such as a sphere or cube.
/// The intersection is computed mathematically.
///
/// A geometry holds the index of a material in a given scene.
pub struct Geometry {
    pub shape: Box<Shape>,
    pub material: MaterialRef
}

impl Primitive for Geometry {
    #[inline]
    fn material<'a>(&self, scene: &'a Scene) -> &'a Material {
        scene.material(self.material)
    }

    fn intersect(&self, ray: &Ray) -> (Intersection, &Primitive) {
        (self.shape.intersect(ray), self)
    }
}
