use crate::space::*;
use crate::{core::bxdf::*, interaction::{SurfaceInteraction, BSDF}};
use super::Material;

pub struct Glass {
    /// Reflection coefficient
    kr: Color,

    /// Trasmission coefficient
    kt: Color,

    /// Refractive index. Typical for glass is 1.5
    eta: f64,

    /// Optional microfacet distribution depending on given roughness parameters
    distribution: Option<MicrofacetDistribution>
}

impl Glass {
    pub fn new(kr: Color, kt: Color, eta: f64, roughness: f64) -> Glass {
        Self::new_uv(kr, kt, eta, roughness, roughness)
    }

    pub fn new_uv(kr: Color, kt: Color, eta: f64, u_roughness: f64, v_roughness: f64) -> Glass {
        let distribution = if u_roughness == 0.0 && v_roughness == 0.0 {
            None
        } else {
            let alphax = MicrofacetDistribution::roughness_to_alpha(u_roughness);
            let alphay = MicrofacetDistribution::roughness_to_alpha(v_roughness);
            Some(MicrofacetDistribution::new(alphax, alphay))
        };

        Glass { kr, kt, eta, distribution }
    }
}

impl Material for Glass {
    fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        let mut bsdf = BSDF::empty(interaction);

        if self.kr != Color::zero() {
            let fresnel = Fresnel::Dielectric(1.0, self.eta);
            let bxdf = if let Some(distribution) = self.distribution {
                BxDF::microfacet_reflection(self.kr, fresnel, distribution)
            } else {
                BxDF::specular_reflection(self.kr, fresnel)
            };
            bsdf.add(bxdf)
        };

        if self.kt != Color::zero() {
            let bxdf = if let Some(distribution) = self.distribution {
                BxDF::microfacet_transmission(self.kt, 1.0, self.eta, TransportMode::Importance, distribution)
            } else {
                BxDF::specular_transmission(self.kt, 1.0, self.eta, TransportMode::Importance)
            };
            bsdf.add(bxdf)
        };

        bsdf
    }
}
