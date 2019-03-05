use crate::space::*;
use super::{util::*, fresnel::Fresnel, TransportMode, BxDFSample};

/// Describes physically plausible specular reflection with the Fresnel model to
/// compute fraction of light that is reflected.
#[derive(Copy, Clone)]
pub struct Reflection {
    /// Reflection spectrum
    r: Color,
    fresnel: Fresnel
}
impl Reflection {
    pub fn new(r: Color, fresnel: Fresnel) -> Reflection {
        Reflection { r, fresnel }
    }

    pub fn sample_f(&self, wo: &Vector, sample: &Point2f) -> BxDFSample {
        // Compute perfect specular reflection direction with normalized shading
        // coordinate axis.
        let wi = Vector::new(-wo.x, -wo.y, wo.z);
        let spectrum = self.fresnel.evaluate(cos_theta(&wi))
            .mul_element_wise(self.r) / abs_cos_theta(&wi);
        BxDFSample::new(spectrum, wi)
    }
}

#[derive(Copy, Clone)]
pub struct Transmission {
    t: Color,
    eta_a: f64,
    eta_b: f64,
    mode: TransportMode,
    fresnel: Fresnel
}
impl Transmission {
    pub fn new(t: Color, eta_a: f64, eta_b: f64, mode: TransportMode) -> Transmission {
        Transmission {
            t, eta_a, eta_b, mode,
            fresnel: Fresnel::Dielectric(eta_a, eta_b)
        }
    }

    pub fn sample_f(&self, wo: &Vector, sample: &Point2f) -> BxDFSample {
        // Determine which eta is incident and which is transmitted
        let entering = cos_theta(wo) > 0.0;
        let (eta_i, eta_t) = if entering {
            (self.eta_a, self.eta_b)
        } else {
            (self.eta_b, self.eta_a)
        };

        // Compute ray direction for specular transmission
        if let Some(wi) = refract(wo, &Normal::new(0.0, 0.0, 1.0), eta_i / eta_t) {
            // TODO: Acount for non-symmetry w/ transmission to different medium
            let spectrum = self.t
                .mul_element_wise(Color::from_value(1.0) - self.fresnel.evaluate(cos_theta(&wi)))
                / abs_cos_theta(&wi);

            BxDFSample::new(spectrum, wi)
        } else {
            BxDFSample::new(Color::zero(), Vector::zero()) // No transmitted light from any direction
        }
    }
}


/// Combined specular reflection and transmission {parameters
#[derive(Copy, Clone)]
pub struct Combined {
    r: Color,
    t: Color,
    eta_a: f64,
    eta_b: f64,
    mode: TransportMode,
    fresnel: Fresnel, // Should always be dielectric (conductors are not usually see-through)
}
impl Combined {
    pub fn new(r: Color, t: Color, eta_a: f64, eta_b: f64, mode: TransportMode) -> Self {
        Combined {
            r, t, eta_a, eta_b, mode,
            fresnel: Fresnel::Dielectric(eta_a, eta_b)
        }
    }
    // TODO: BxDFSample F
}
