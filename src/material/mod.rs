use crate::{space::*, interaction::SurfaceInteraction, Accel};

pub trait Material {

    /// Get the colour of the material at the given point of interaction. Use
    /// the scene root node for reference
    fn color(&self, interaction: &SurfaceInteraction, root: &Accel) -> Color;
}

pub mod phong;
