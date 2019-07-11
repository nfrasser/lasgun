use cgmath::{Deg, Rad};
use std::f64::consts::FRAC_1_PI;
use crate::space::*;
use super::util::*;

/// Non-physically-based Lambertian diffuse reflection
#[derive(Copy, Clone)]
pub struct Lambertian {
    r: Color
}

impl Lambertian {
    pub fn new(r: Color) -> Lambertian { Lambertian { r } }
    pub fn f(&self) -> Color { self.r * FRAC_1_PI }
    pub fn rho(&self) -> Color { self.r }
}

/// Oren-Nayar diffuse reflection
#[derive(Copy, Clone)]
pub struct OrenNayar {
    /// Reflection specturm
    r: Color,
    a: f64,
    b: f64
}

impl OrenNayar {
    pub fn new(r: Color, sigma: f64) -> OrenNayar {
        let sigma = Rad::from(Deg(sigma)).0; // Convert to radians
        let sigma2 = sigma * sigma;
        let a = 1.0 - (sigma2 / 2.0 * (sigma2 + 0.33));
        let b = 0.45 * sigma2 / (sigma2 + 0.09);
        OrenNayar { r, a, b }
    }

    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        let sin_theta_i = sin_theta(wi);
        let sin_theta_o = sin_theta(wo);

        // Compute cos of Oren-Nayar model
        let max_cos = if sin_theta_i > 1e-4 && sin_theta_o > 1e-4 {
            let sin_phi_i = sin_phi(wi); let cos_phi_i = cos_phi(wi);
            let sin_phi_o = sin_phi(wo); let cos_phi_o = cos_phi(wo);
            let d_cos = cos_phi_i * cos_phi_o + sin_phi_i * sin_phi_o;
            d_cos.max(0.0)
        } else { 0.0 };

        // Compute sin and tan of Oren-Nayar model
        let (sin_alpha, tan_beta) = if abs_cos_theta(wi) > abs_cos_theta(wo) {
            (sin_theta_o, sin_theta_i / abs_cos_theta(wi))
        } else {
            (sin_theta_i, sin_theta_o / abs_cos_theta(wo))
        };

        self.r * FRAC_1_PI * (self.a + self.b * max_cos * sin_alpha * tan_beta)
    }
}
