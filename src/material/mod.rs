use crate::{interaction::{SurfaceInteraction, BSDF}, Accel};

pub trait Material {

    /// Computes the function for how light is handled at the material at the
    /// given point of interaction. Use the scene root node for reference.
    fn scattering(&self, interaction: &SurfaceInteraction, root: &Accel) -> BSDF;
}

pub mod background;
pub mod matte;
pub mod plastic;
pub mod metal;
pub mod mirror;
