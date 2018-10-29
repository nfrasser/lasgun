use crate::{
    space::*,
    scene::Scene,
    primitive::Primitive,
    ray::Ray,
    interaction::SurfaceInteraction
};

use std::marker::{Sync, Send};

pub trait Material: Sync + Send {

    /**
        Get the colour of the material at point q as observed from point e
        where the surface at q has normal n in the given scene
    */
    fn color(
        &self,
        ray: &Ray, // Incident ray
        interaction: &SurfaceInteraction, // interaction at the surface
        scene: &Scene, // The scene and root, for reference/refraction
        root: &dyn Primitive
    ) -> Color;
}

pub mod phong;
pub mod refractive;
