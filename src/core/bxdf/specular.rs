use crate::space::*;
use super::{util::*, fresnel::Substance, LightSample};

/// Describes physically plausible specular reflection with the Substance model to
/// compute fraction of light that is reflected.
#[derive(Copy, Clone)]
pub struct Reflection {
    /// Reflection spectrum
    r: Color,
    substance: Substance
}
impl Reflection {
    pub fn new(r: Color, substance: Substance) -> Reflection {
        Reflection { r, substance }
    }

    pub fn sample_f(&self, wo: &Vector, _sample: &Point2f) -> LightSample {
        // Compute perfect specular reflection direction with normalized shading
        // coordinate axis.
        let wi = Vector::new(-wo.x, -wo.y, wo.z);
        let spectrum = self.substance.evaluate(cos_theta(&wi))
            .mul_element_wise(self.r) / abs_cos_theta(&wi);
        LightSample::new(spectrum, wi, 1.0)
    }
}

#[derive(Copy, Clone)]
pub struct Transmission {
    t: Color,
    eta_a: f64,
    eta_b: f64,
    // mode: TransportMode, // TODO
    substance: Substance
}
impl Transmission {
    pub fn new(t: Color, eta_a: f64, eta_b: f64) -> Transmission {
        Transmission {
            t, eta_a, eta_b,
            substance: Substance::Dielectric(eta_a, eta_b)
        }
    }

    pub fn sample_f(&self, wo: &Vector, _sample: &Point2f) -> LightSample {
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
                .mul_element_wise(Color::from_value(1.0) - self.substance.evaluate(cos_theta(&wi)))
                / abs_cos_theta(&wi);

            LightSample::new(spectrum, wi, 1.0)
        } else {
            LightSample::zero() // No transmitted light from any direction
        }
    }
}


/*
/// Combined specular reflection and transmission parameters
/// TODO
#[derive(Copy, Clone)]
pub struct Combined {
    r: Color,
    t: Color,
    eta_a: f64,
    eta_b: f64,
    mode: TransportMode,
    substance: Substance, // Should always be dielectric (conductors are not usually see-through)
}
impl Combined {
    pub fn new(r: Color, t: Color, eta_a: f64, eta_b: f64, mode: TransportMode) -> Self {
        Combined {
            r, t, eta_a, eta_b, mode,
            substance: Substance::Dielectric(eta_a, eta_b)
        }
    }
    // TODO: LightSample F
}
*/
