use crate::{
    space::*,
    ray::Ray,
    shape::Shape,
    interaction::SurfaceInteraction,
    scene::MaterialRef,
};

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
    fn bound(&self) -> Bounds { self.shape.bound() }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> Option<&dyn Primitive> {
        if let Some(primitive) = self.shape.intersect(ray, interaction) {
            interaction.material = Some(self.material);
            Some(primitive)
        } else {
            None
        }
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.shape.intersects(ray)
    }
}
