use crate::{space::*, interaction::SurfaceInteraction, primitive::Primitive, Accel};

use std::marker::{Sync, Send};

pub trait Material: Sync + Send {

    /// Get the colour of the material at the given point of interaction. Use
    /// the scene root node for reference
    fn color(&self, primitive: &dyn Primitive, interaction: &SurfaceInteraction, root: &Accel) -> Color;
}

pub mod background;
pub mod phong;
pub mod refractive;
