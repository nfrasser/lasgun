
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
pub struct Geometry<S: Shape> {
    pub shape: S,
    pub material: MaterialRef
}

impl<S: Shape> Primitive for Geometry<S> {
    #[inline]
    fn material<'a>(&self, scene: &'a Scene) -> &'a dyn Material {
        scene.material(self.material)
    }

    #[inline]
    fn intersect(&self, ray: &Ray) -> (Intersection, &dyn Primitive) {
        (self.shape.intersect(ray), self)
    }
}
