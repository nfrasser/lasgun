use crate::space::*;
use crate::{scene::Scene, primitive::Primitive};

pub trait Material {

    /**
        Get the colour of the material at point q as observed from point e
        where the surface at q has normal n in the given scene
    */
    fn color(
        &self,
        q: &Point, // Point on the scene to be lit
        e: &Point, // Eye position
        n: &Normal, // Normal to the point and surface
        scene: &Scene, // The scene, for reference
        root: &dyn Primitive
    ) -> Color;
}

pub mod phong;
