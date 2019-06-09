use crate::space::*;
use crate::{core::bxdf::BxDF, interaction::{SurfaceInteraction, BSDF}, Accel};
use super::Material;

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

    pub fn quick(kd: Color) -> Matte {
        Matte::new(kd, 0.0)
    }
}

impl Material for Matte {
    fn scattering(&self, interaction: &SurfaceInteraction, _root: &Accel) -> BSDF {
        BSDF::new(interaction, &[
            if self.sigma == 0.0 {
                BxDF::quick_diffuse(self.kd)
            } else {
                BxDF::diffuse(self.kd, self.sigma)
            }
        ])
    }
}
