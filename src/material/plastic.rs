use crate::space::*;
use crate::{core::bxdf::*, interaction::{SurfaceInteraction, BSDF}};

#[derive(Debug, Copy, Clone)]
pub struct Plastic {
    /// Diffuse coefficient
    kd: Color,

    /// Specular coefficient
    ks: Color,

    roughness: f64
}

impl Plastic {
    pub fn new(kd: Color, ks: Color, roughness: f64) -> Plastic {
        Plastic { kd, ks, roughness }
    }

    pub fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        let mut bsdf = BSDF::empty(interaction);

        // Diffuse component
        if self.kd != Color::zero() {
            bsdf.add(BxDF::quick_diffuse(self.kd))
        };

        // Don't add ks if it doesn't contrinbute
        if self.ks != Color::zero() {
            let rough = self.roughness;
            let substance = Substance::Dielectric(1.0, 1.5);
            let distribution = MicrofacetDistribution::new(rough, rough);
            bsdf.add(BxDF::microfacet_reflection(self.ks, substance, distribution));
        };

        bsdf
    }
}
