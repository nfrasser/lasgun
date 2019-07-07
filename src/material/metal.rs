use crate::space::*;
use crate::{core::bxdf::*, interaction::{SurfaceInteraction, BSDF}};
use super::Material;

pub struct Metal {
    eta: Color,
    k: Color,
    u_roughness: f64,
    v_roughness: f64
}

impl Metal {
    pub fn new(eta: Color, k: Color, roughness: f64) -> Metal {
        Metal { eta, k, u_roughness: roughness, v_roughness: roughness }
    }

    pub fn new_uv(eta: Color, k: Color, u_roughness: f64, v_roughness: f64) -> Metal {
        Metal { eta, k, u_roughness, v_roughness }
    }
}

impl Material for Metal {
    fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        let mut bsdf = BSDF::empty(interaction);

        // Microfacet conductor component
        let white = Color::from_value(1.0);
        let substance = Substance::Conductor(white, self.eta, self.k);
        let distribution = MicrofacetDistribution::new(self.u_roughness, self.v_roughness);
        bsdf.add(BxDF::microfacet_reflection(white, substance, distribution));
        bsdf
    }
}
