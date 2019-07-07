use std::{f64::consts::PI, ops::Neg};
use crate::space::*;
use super::{util::*, sampling::*, fresnel::Substance, TransportMode, BxDFSample};

/// Trowbridge-Reitz microfacet distribution model.
#[derive(Copy, Clone)]
pub struct Distribution {
    pub alphax: f64,
    pub alphay: f64
}

impl Distribution {
    #[inline]
    pub fn roughness_to_alpha(roughness: f64) -> f64 {
        // Thanks PBRT <3
        // https://github.com/mmp/pbrt-v3/blob/9f717d847a807793fa966cf0eaa366852efef167/src/core/microfacet.h#L122-L128
        let roughness = roughness.max(1e-3 as f64);
        let x = roughness.ln();
        1.62142 + 0.819955 * x +
            0.1734 * x * x +
            0.0171201 * x * x * x +
            0.000640711 * x * x * x * x
    }

    #[inline]
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

    /// Compute Probability distribution function
    fn pdf(&self, _wo: &Vector, wh: &Vector) -> f64 {
        self.d(wh) * abs_cos_theta(wh)
    }

    // Get sample reflected direction
    fn sample_wh(&self, wo: &Vector, sample: &Point2f) -> Vector {
        let mut cos_theta = 0.0;
        let mut phi = (2.0 * PI) * sample.y;

        if self.alphax == self.alphay {
            let tan_theta2 = self.alphax * self.alphax * sample.x
                / (1.0 - sample.x);
            cos_theta = 1.0 / (1.0 + tan_theta2).sqrt();
        } else {
            phi = (self.alphay / self.alphax * (2.0 * PI * sample.y + 0.5 * PI).tan()).atan();
            if sample.y > 0.5 { phi += PI };
            let (sin_phi, cos_phi) = (phi.sin(), phi.cos());
            let alphax2 = self.alphax * self.alphax;
            let alphay2 = self.alphay * self.alphay;

            let alpha2 = 1.0 / (cos_phi * cos_phi / alphax2 + sin_phi * sin_phi / alphay2);
            let tan_theta2 = alpha2 * sample.x / (1.0 - sample.x);
            cos_theta = 1.0 / (1.0 + tan_theta2).sqrt();
        }
        let sin_theta = (1.0 - cos_theta * cos_theta).max(0.0).sqrt();
        let wh = spherical_direction(sin_theta, cos_theta, phi);
        if same_hemisphere(wo, &wh) { wh } else { -wh }
    }
}

/// Torrence-Sparrow Microfacet Reflection model, implementing the
/// Trowbridge-Reitz microfacet distribution model.
#[derive(Copy, Clone)]
pub struct Reflection {
    /// Reflection specturm
    r: Color,

    /// Surface reflection model
    substance: Substance,

    /// Common Trowbridge-Reitz model code
    distribution: Distribution,
}
impl Reflection {
    pub fn new(r: Color, substance: Substance, distribution: Distribution) -> Reflection {
        Reflection { r, substance, distribution }
    }

    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        let cos_theta_o = abs_cos_theta(wo);
        let cos_theta_i = abs_cos_theta(wi);
        let wh = wi + wo;

        // Handle degenerate cases for microfacet relection
        if cos_theta_i == 0.0 || cos_theta_o == 0.0 { return Color::zero() };
        if wh.x == 0.0 && wh.y == 0.0 && wh.z == 0.0 { return Color::zero() };

        let wh = wh.normalize();
        let spectrum = self.substance.evaluate(wi.dot(wh));
        (self.r * self.distribution.d(&wh) * self.distribution.g(wo, wi))
            .mul_element_wise(spectrum)
            / ( 4.0 * cos_theta_i * cos_theta_o)
    }

    pub fn sample_f(&self, wo: &Vector, sample: &Point2f) -> BxDFSample {
        // Sample microfacet orientation wh and reflected direction wi
        if wo.z == 0.0 { return BxDFSample::zero() };
        let wh = self.distribution.sample_wh(wo, sample);
        let wi = reflect(wo, &wh);
        if !same_hemisphere(wo, &wi) {
            return BxDFSample::new(Color::zero(), wi, 0.0)
        }

        // Compute PDF of wi for microfacet reflection
        let pdf = self.distribution.pdf(wo, &wh) / (4.0 * wo.dot(wh));
        BxDFSample::new(self.f(wo, &wi), wi, pdf)
    }

    pub fn pdf(&self, wo: &Vector, wi: &Vector) -> f64 {
        if !same_hemisphere(wo, wi) { return 0.0 }
        let wh = (wo + wi).normalize();
        self.distribution.pdf(wo, &wh) / (4.0 * wo.dot(wh))
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
    substance: Substance,

    /// Common Trowbridge-Reitz model code
    distribution: Distribution,
}
impl Transmission {
    pub fn new(t: Color, eta_a: f64, eta_b: f64, mode: TransportMode, distribution: Distribution)
    -> Transmission {
        Transmission {
            t, eta_a, eta_b, mode,
            substance: Substance::Dielectric(eta_a, eta_b),
            distribution
        }
    }
    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        let eta = self.eta(wo);
        let wh = wo + eta * wi;

        let spectrum = self.substance.evaluate(cos_theta(wo));
        (eta * eta * self.distribution.d(&wh) * self.distribution.g(wo, wi) * (Color::from_value(1.0) - spectrum))
            / (wo.dot(wh) + eta * wi.dot(wh)).powi(2)
            * (wi.dot(wh).abs() * wo.dot(wh).abs())
            / (cos_theta(wo) * cos_theta(wi))
    }

    pub fn sample_f(&self, wo: &Vector, sample: &Point2f) -> BxDFSample {
        if wo.z == 0.0 { return BxDFSample::zero() };
        let wh = self.distribution.sample_wh(wo, sample);
        let eta = self.eta(wo);

        if let Some(wi) = refract(wo, &normal::Normal3(wh), eta) {
            BxDFSample::new(self.f(wo, &wi), wi, self.pdf(wo, &wi))
        } else {
            BxDFSample::zero()
        }
    }

    pub fn pdf(&self, wo: &Vector, wi: &Vector) -> f64 {
        if same_hemisphere(wo, wi) { return 0.0 };

        // Compute wh from wo and wi for microfacet transmission
        let eta = self.eta(wo);
        let wh = (wo + eta * wi).normalize();

        // Compute change of variables dwh_dwi for microfacet transmission
        let sqrt_denom = wo.dot(wh) + eta * wi.dot(wh);
        let dwh_dwi = ((eta * eta * wi.dot(wh)) / (sqrt_denom * sqrt_denom)).abs();
        self.distribution.pdf(wo, &wh) * dwh_dwi
    }

    /// Compute reflectance eta based on outgoing direction
    #[inline]
    fn eta(&self, wo: &Vector) -> f64 {
        if cos_theta(wo) > 0.0 {
            // Entering
            self.eta_b / self.eta_a
        } else {
            // Exiting
            self.eta_a / self.eta_b
        }
    }
}
