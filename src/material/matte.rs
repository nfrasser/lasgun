use crate::space::Color;
use crate::{core::bxdf::BxDF, interaction::{SurfaceInteraction, BSDF}};

#[derive(Debug, Copy, Clone)]
pub struct Matte {
    /// Surface reflection value
    kd: Color,

    /// Scalar roughness value, for Oren-Nayar model
    sigma: f64
}

impl Matte {
    pub fn new(kd: Color, sigma: f64) -> Matte {
        Matte { kd, sigma: sigma.max(0.0).min(90.0) }
    }

    pub fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        BSDF::new(interaction, &[
            if self.sigma == 0.0 {
                BxDF::quick_diffuse(self.kd)
            } else {
                BxDF::diffuse(self.kd, self.sigma)
            }
        ])
    }
}
