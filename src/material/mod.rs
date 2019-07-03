use crate::{interaction::{SurfaceInteraction, BSDF}};

pub trait Material {

    /// Computes the function for how light is handled at the material at the
    /// given point of interaction.
    fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF;
}

pub mod background;
pub mod matte;
pub mod plastic;
pub mod metal;
pub mod mirror;
