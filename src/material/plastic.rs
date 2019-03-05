use crate::space::*;
use crate::{core::bxdf::*, interaction::{SurfaceInteraction, BSDF}, Accel};
use super::Material;

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
}

impl Material for Plastic {
    fn scattering(&self, interaction: &SurfaceInteraction, root: &Accel) -> BSDF {
        let mut bsdf = BSDF::empty(interaction);

        // Diffuse component
        if self.kd != Color::zero() {
            bsdf.add(BxDF::quick_diffuse(self.kd))
        };

        // Don't add ks if it doesn't contrinbute
        if self.ks != Color::zero() {
            let rough = self.roughness;
            let fresnel = Fresnel::Dielectric(1.0, 1.5);
            let distribution = MicrofacetDistribution::new(rough, rough);
            bsdf.add(BxDF::microfacet_reflection(self.ks, fresnel, distribution))
        };

        bsdf
    }
}
