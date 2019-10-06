use crate::space::*;
use crate::{core::bxdf::*, interaction::{SurfaceInteraction, BSDF}};

#[derive(Debug, Copy, Clone)]
pub struct Mirror {
    /// Reflection coefficient
    kr: Color
}

impl Mirror {
    pub fn new(kr: Color) -> Mirror {
        Mirror { kr }
    }

    pub fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        BSDF::new(interaction, &[BxDF::specular_reflection(self.kr, Substance::NoOp)])
    }
}
