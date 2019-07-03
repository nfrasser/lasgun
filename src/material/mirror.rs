use crate::space::*;
use crate::{core::bxdf::*, interaction::{SurfaceInteraction, BSDF}};
use super::Material;

pub struct Mirror {
    /// Reflection coefficient
    kr: Color
}

impl Mirror {
    pub fn new(kr: Color) -> Mirror {
        Mirror { kr }
    }
}

impl Material for Mirror {
    fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        BSDF::new(interaction, &[BxDF::specular_reflection(self.kr, Fresnel::NoOp)])
    }
}
