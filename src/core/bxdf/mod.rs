/// Bidirectional Reflectance/Transmittance Distribution function
/// implementation: Calculate the amount of light at a surface without
/// accounting for sub-surface scattering.

use crate::space::*;

mod fresnel;
mod specular;
mod diffuse;
mod microfacet;

pub use self::fresnel::Substance;

pub type MicrofacetDistribution = microfacet::Distribution;

bitflags! {
    pub struct BxDFType: u32 {
        const NONE = 0;
        const REFLECTION = 1 << 0;
        const TRANSMISSION = 1 << 1;
        const DIFFUSE = 1 << 2;
        const GLOSSY = 1 << 3;
        const SPECULAR = 1 << 4;
        const ALL
            = Self::REFLECTION.bits
            | Self::TRANSMISSION.bits
            | Self::DIFFUSE.bits
            | Self::GLOSSY.bits
            | Self::SPECULAR.bits;
    }
}

/// Used for Asymetric BSDFs (involving reflection/refraction)
#[derive(Copy, Clone)]
pub enum TransportMode { Radiance, Importance }

/// Details about the sample for a BxDF integral in some direction
pub struct BxDFSample {
    pub spectrum: Color,
    pub wi: Vector,
    pub pdf: f64,
    // pub t: BxDFType // for future use?
}
impl BxDFSample {
    #[inline]
    pub fn new(spectrum: Color, wi: Vector, pdf: f64) -> BxDFSample {
        BxDFSample { spectrum, wi, pdf }
    }

    #[inline]
    pub fn zero() -> BxDFSample {
        Self::new(Color::zero(), Vector::zero(), 0.0)
    }
}

/// Specifications for different kinds of generic Bidirectional
/// Reflectance/Transmittance Distribution Functions, used in Material
/// definitions.
#[derive(Copy, Clone)]
pub enum BxDF {
    /// Where f always evaluates to the same thing. Used for backgrounds.
    Constant(Color),

    /// With specular reflection. First parameter is reflection specturm and
    /// second is Substance light transport model.
    SpecularReflection(specular::Reflection),

    /// With specular transmission.
    SpecularTransmission(specular::Transmission),

    /// Combined specular reflection and transmission, used with Monte-Carlo
    /// integrator.
    Specular(specular::Combined),

    /// Less physically-accurate Lambertain diffuse, for when Oren-Nayar sigma
    /// parameter is zero
    QuickDiffuse(diffuse::Lambertian),

    /// Diffuse reflection, using more physically accurate OrenNayar model
    /// compared to Lambertian.
    Diffuse(diffuse::OrenNayar),

    /// Microfacet reflection with Trowbridge-Reitz distribution implementation.
    MicrofacetReflection(microfacet::Reflection),

    /// Microfacet transmission with Trowbridge-Reitz distribution implementation.
    MicrofacetTransmission(microfacet::Transmission),

    // Function with scaled (partial) contribution, given by the color.
    // Scaled(Box<BxDF>, Color),
}

impl BxDF {
    pub fn specular_reflection(r: Color, substance: Substance) -> BxDF {
        let reflection = specular::Reflection::new(r, substance);
        BxDF::SpecularReflection(reflection)
    }

    pub fn specular_transmission(t: Color, eta_a: f64, eta_b: f64, mode: TransportMode) -> BxDF {
        let transmission = specular::Transmission::new(t, eta_a, eta_b, mode);
        BxDF::SpecularTransmission(transmission)
    }

    pub fn specular(r: Color, t: Color, eta_a: f64, eta_b: f64, mode: TransportMode) -> BxDF {
        let specular = specular::Combined::new(r, t, eta_a, eta_b, mode);
        BxDF::Specular(specular)
    }

    pub fn quick_diffuse(r: Color) -> BxDF {
        BxDF::QuickDiffuse(diffuse::Lambertian::new(r))
    }

    pub fn diffuse(r: Color, sigma: f64) -> BxDF {
        BxDF::Diffuse(diffuse::OrenNayar::new(r, sigma))
    }

    pub fn microfacet_reflection(r: Color, substance: Substance, distribution: microfacet::Distribution) -> BxDF {
        let reflection = microfacet::Reflection::new(r, substance, distribution);
        BxDF::MicrofacetReflection(reflection)
    }

    pub fn microfacet_transmission(t: Color, eta_a: f64, eta_b: f64, mode: TransportMode, distribution: microfacet::Distribution) -> BxDF {
        let transmission =
            microfacet::Transmission::new(t, eta_a, eta_b, mode, distribution);
        BxDF::MicrofacetTransmission(transmission)
    }

    // pub fn scaled(bxdf: BxDF, spectrum: Color) -> BxDF {
    //     BxDF::Scaled(Box::new(bxdf), spectrum)
    // }

    /// Type
    pub fn t(&self) -> BxDFType {
        match self {
            BxDF::Constant(_) => BxDFType::NONE,
            BxDF::SpecularReflection(_) => BxDFType::REFLECTION | BxDFType::SPECULAR,
            BxDF::SpecularTransmission(_) => BxDFType::TRANSMISSION | BxDFType::SPECULAR,
            BxDF::Specular(_) => BxDFType::REFLECTION | BxDFType::TRANSMISSION | BxDFType::SPECULAR,
            BxDF::QuickDiffuse(_) => BxDFType::REFLECTION | BxDFType::DIFFUSE,
            BxDF::Diffuse(_) => BxDFType::REFLECTION | BxDFType::DIFFUSE,
            BxDF::MicrofacetReflection(_) => BxDFType::REFLECTION | BxDFType::GLOSSY,
            BxDF::MicrofacetTransmission(_) => BxDFType::TRANSMISSION | BxDFType::GLOSSY,
            // BxDF::Scaled(bxdf, _) => bxdf.t(),
        }
    }

    pub fn matches(&self, flags: BxDFType) -> bool {
        let t = self.t();
        (t & flags) == t
    }

    pub fn has_t(&self, flags: BxDFType) -> bool {
        self.t() & flags != BxDFType::NONE
    }

    /// Evaluate the distribution function for outgoing vector wo and incident
    /// direction wi. Actual value, not a sample or estimate.
    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        match self {
            BxDF::Constant(spectrum) => *spectrum,
            BxDF::QuickDiffuse(d) => d.f(),
            BxDF::Diffuse(d) => d.f(wo, wi),
            BxDF::MicrofacetReflection(r) => r.f(wo, wi),
            BxDF::MicrofacetTransmission(t) => t.f(wo, wi),
            // BxDF::Scaled(bxdf, scale) => scale.mul_element_wise(bxdf.f(wo, wi)),
            _ => Color::zero(), // Specular has no scattering, only sampling
        }
    }

    /// BxDFSample the value of f - call this multiple times and average to
    /// approximate the evaluation of the BxDF integral (i.e., Monte Carlo
    /// Integration). Also returns the type of the resolved sample.
    pub fn sample_f(&self, wo: &Vector, sample: &Point2f) -> BxDFSample {
        match self {
            BxDF::SpecularReflection(r) => r.sample_f(wo, sample),
            BxDF::SpecularTransmission(t) => t.sample_f(wo, sample),
            BxDF::MicrofacetReflection(r) => r.sample_f(wo, sample),
            BxDF::MicrofacetTransmission(t) => t.sample_f(wo, sample),
            _ => {
                // Cosine-sample the hemisphere, flipping the direction if necessary
                let mut wi = sampling::cosine_sample_hemisphere(sample);
                if wo.z < 0.0 { wi.z *= -1.0 };
                let spectrum = self.f(wo, &wi);
                let pdf = sampling::pdf(wo, &wi);
                BxDFSample::new(spectrum, wi, pdf)
            }
        }
    }

    /// Probability density function for reflectance of light
    pub fn pdf(&self, wo: &Vector, wi: &Vector) -> f64 {
        match self {
            BxDF::MicrofacetReflection(r) => r.pdf(wo, wi),
            BxDF::MicrofacetTransmission(t) => t.pdf(wo, wi),
            BxDF::SpecularReflection(_) => 0.0,
            BxDF::SpecularTransmission(_) => 0.0,
            _ => sampling::pdf(wo, wi)
        }
    }

    /// Hemispherical-Directional Reflectance funtion gives total reflection in
    /// a given direction due to constant illumination over the hemisphere
    /// (which happens to also be equivalent to reflection in all directions
    /// based in light from a single incoming direction).
    pub fn rho_hd(&self, wo: &Vector, wi: &Vector, samples: &[Point2f]) -> Color {
        Color::zero()
    }

    /// Hemispherical-Hemispherical Reflectance funtion gives fraction of light
    /// reflected by a surface when incident light is the same from all
    /// directions.
    pub fn rho_hh(&self, samples1: &[Point2f], samples2: &[Point2f]) -> Color {
        Color::zero()
    }
}

/// Utility functions
///
/// Geometric/Trig functions with a normalized coordinate system for shading
/// where n = (0, 0, 1).
///
/// - `theta` is the angle between a direction vector an a surface normal
/// - `phi` is the angle between the x shading coordinate axis and the
///   projection of the direction vector onto the tangent plane (which also
///   contains the x axis).
pub mod util {
    use crate::space::*;

    #[inline] pub fn cos_theta(w: &Vector) -> f64 { w.z }
    #[inline] pub fn cos2_theta(w: &Vector) -> f64 { w.z * w.z }
    #[inline] pub fn abs_cos_theta(w: &Vector) -> f64 { w.z.abs() }

    #[inline] pub fn sin2_theta(w: &Vector) -> f64 { (1.0 - cos2_theta(w)).max(0.0) }
    #[inline] pub fn sin_theta(w: &Vector) -> f64 { sin2_theta(w).sqrt() }

    #[inline] pub fn tan_theta(w: &Vector) -> f64 { sin_theta(w) / cos_theta(w) }
    #[inline] pub fn tan2_theta(w: &Vector) -> f64 { sin2_theta(w) / cos2_theta(w) }

    #[inline] pub fn cos_phi(w: &Vector) -> f64 {
        let sin_theta = sin_theta(w);
        if sin_theta == 0.0 { 1.0 } else { (w.x / sin_theta).max(-1.0).min(1.0) }
    }

    #[inline] pub fn sin_phi(w: &Vector) -> f64 {
        let sin_theta = sin_theta(w);
        if sin_theta == 0.0 { 0.0 } else { (w.y / sin_theta).max(-1.0).min(1.0) }
    }

    #[inline] pub fn cos2_phi(w: &Vector) -> f64 { cos_phi(w) * cos_phi(w) }
    #[inline] pub fn sin2_phi(w: &Vector) -> f64 { sin_phi(w) * sin_phi(w) }

    // Delta Phi, between two vectors in shading coordinate system
    #[inline] pub fn cos_d_phi(wa: &Vector, wb: &Vector) -> f64 {
        let result = (wa.x * wb.x + wa.y * wb.y)
            / ((wa.x * wa.x + wa.y * wa.y) * (wb.x * wb.x + wb.y * wb.y)).sqrt();
        result.max(-1.0).min(1.0)
    }

    #[inline] pub fn reflect(wo: &Vector, n: &Vector) -> Vector {
        -1.0 * wo + 2.0 * wo.dot(*n) * n
    }

    /// Refract the given incident direction vector into a medium with the
    /// refractive index `eta`. Returns `None` if the refraction fails due to Total
    /// Internal Reflection.
    #[inline] pub fn refract(wi: &Vector, n: &Normal, eta: f64) -> Option<Vector> {
        // Compute cos_theta_t w/ Snell's law
        let n = n.0;
        let cos_theta_i = n.dot(*wi);
        let sin2_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0);
        let sin2_theta_t = eta * eta * sin2_theta_i;

        // Handle total internal reflection
        if sin2_theta_t >= 1.0 { return None }

        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
        Some(eta * -1.0 * wi + (eta * cos_theta_i - cos_theta_t) * n)
    }
}

// Private sampling utilities used to determine light distribution
mod sampling {
    use super::util::*;
    use crate::space::*;
    use std::f64::consts::{FRAC_1_PI, FRAC_PI_2, FRAC_PI_4};

    /// Default PDF given incoming vector for BxDFs
    #[inline] pub fn pdf(wo: &Vector, wi: &Vector) -> f64 {
        if same_hemisphere(wo, wi) { abs_cos_theta(wi) * FRAC_1_PI } else { 0.0 }
    }

    #[inline] pub fn same_hemisphere(w: &Vector, wp: &Vector) -> bool { w.z * wp.z > 0.0 }

    #[inline] pub fn spherical_direction(sin_theta: f64, cos_theta: f64, phi: f64) -> Vector {
        Vector::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta)
    }

    #[inline] pub fn cosine_sample_hemisphere(u: &Point2f) -> Vector {
        let d = concentric_sample_disk(u);
        let z = (1.0 - d.x * d.x - d.y * d.y).max(0.0).sqrt();
        Vector::new(d.x, d.y, z)
    }

    fn concentric_sample_disk(u: &Point2f) -> Point2f {
        // Map uniform random numbers to $[-1,1]^2$
        let u_offset = 2.0 * u - Vector2f::new(1.0, 1.0);

        // Handle degeneracy at the origin
        if u_offset.x == 0.0 && u_offset.y == 0.0 {
            return Point2f::new(0.0, 0.0);
        }

        // Apply concentric mapping to point
        let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
            (u_offset.x, FRAC_PI_4 * (u_offset.y / u_offset.x))
        } else {
            (u_offset.y, FRAC_PI_2 - FRAC_PI_4 * (u_offset.x / u_offset.y))
        };

        r * Point2f::new(theta.cos(), theta.sin())
    }
}
