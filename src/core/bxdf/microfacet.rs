use std::f64::consts::PI;
use crate::space::*;
use super::{util::*, fresnel::Fresnel, TransportMode};

/// Trowbridge-Reitz microfacet distribution model.
#[derive(Copy, Clone)]
pub struct Distribution {
    pub alphax: f64,
    pub alphay: f64
}

impl Distribution {
    pub fn new(alphax: f64, alphay: f64) -> Distribution {
        Distribution { alphax, alphay }
    }

    /// Gives differenctial area of microfaces w/ the surface normal wh
    fn d(&self, wh: &Vector) -> f64 {
        let tan2_theta = tan2_theta(wh);
        if tan2_theta.is_infinite() { return 0.0 };
        let cos4_theta = cos2_theta(wh) * cos2_theta(wh);
        let e = (
            cos2_phi(wh) / (self.alphax * self.alphax) +
            sin2_phi(wh) / (self.alphay * self.alphay)
        ) * tan2_theta;
        1.0 / (PI * self.alphax * self.alphay * cos4_theta * (1.0 + e) * (1.0 + e))
    }

    /// Gives fraction of microfacets in a differential area that are visible
    /// from both diretion w0 and wi.
    fn g(&self, wo: &Vector, wi: &Vector) -> f64 {
        1.0 / (1.0 + self.lambda(wo) + self.lambda(wi))
    }

    /// Masking-Shadow Function gives the fraction of microfacets with normal wh
    /// that are visible from direction w.
    fn g1(&self, w: &Vector) -> f64 {
        1.0 / (1.0 + self.lambda(w))
    }
    /// Measures ratio of invisible v.s. visible microfacets based on viewing
    /// angle. Used to compute shadow masking function.
    fn lambda(&self, w: &Vector) -> f64 {
        let abs_tan_theta = tan_theta(w).abs();
        if abs_tan_theta.is_infinite() { return 0.0; }

        // Compute alpha for direction w
        let alpha = (
            cos2_phi(w) * self.alphax * self.alphax +
            sin2_phi(w) * self.alphay * self.alphay
        ).sqrt();
        let alpha2_tan2_theta = (alpha * abs_tan_theta) * (alpha * abs_tan_theta);
        ((1.0 + alpha2_tan2_theta).sqrt() - 1.0) / 2.0
    }
}

/// Torrence-Sparrow Microfacet Reflection model, implementing the
/// Trowbridge-Reitz microfacet distribution model.
#[derive(Copy, Clone)]
pub struct Reflection {
    /// Reflection specturm
    r: Color,

    /// Surface reflection model
    fresnel: Fresnel,

    /// Common Trowbridge-Reitz model code
    distribution: Distribution,
}
impl Reflection {
    pub fn new(r: Color, fresnel: Fresnel, distribution: Distribution) -> Reflection {
        Reflection { r, fresnel, distribution }
    }

    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        let cos_theta_o = abs_cos_theta(wo);
        let cos_theta_i = abs_cos_theta(wi);
        let wh = wi + wo;

        // Handle degenerate cases for microfacet relection
        if cos_theta_i == 0.0 || cos_theta_o == 0.0 { return Color::zero() };
        if wh.x == 0.0 && wh.y == 0.0 && wh.z == 0.0 { return Color::zero() };

        let wh = wh.normalize();
        let spectrum = self.fresnel.evaluate(wi.dot(wh));
        (self.r * self.distribution.d(&wh) * self.distribution.g(wo, wi))
            .mul_element_wise(spectrum)
            / ( 4.0 * cos_theta_i * cos_theta_o)
    }
}

/// Torrence-Sparrow Microfacet Reflection model, implementing the
/// Trowbridge-Reitz microfacet distribution model.
#[derive(Copy, Clone)]
pub struct Transmission {
    /// Transmission spectrum
    t: Color,
    eta_a: f64,
    eta_b: f64,

    mode: TransportMode,

    /// Surface reflection model
    fresnel: Fresnel,

    /// Common Trowbridge-Reitz model code
    distribution: Distribution,
}
impl Transmission {
    pub fn new(t: Color, eta_a: f64, eta_b: f64, mode: TransportMode, distribution: Distribution)
    -> Transmission {
        Transmission {
            t, eta_a, eta_b, mode,
            fresnel: Fresnel::Dielectric(eta_a, eta_b),
            distribution
        }
    }
    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        let entering = cos_theta(wo) > 0.0;
        let (eta_i, eta_t) = if entering {
            (self.eta_a, self.eta_b)
        } else {
            (self.eta_b, self.eta_a)
        };
        let eta = eta_i / eta_t;
        let wh = wo + eta * wi;

        let spectrum = self.fresnel.evaluate(cos_theta(wo));
        (eta * eta * self.distribution.d(&wh) * self.distribution.g(wo, wi) * (Color::from_value(1.0) - spectrum))
            / (wo.dot(wh) + eta * wi.dot(wh)).powi(2)
            * (wi.dot(wh).abs() * wo.dot(wh).abs())
            / (cos_theta(wo) * cos_theta(wi))
    }
}
