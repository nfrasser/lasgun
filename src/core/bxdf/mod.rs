/// Bidirectional Reflectance/Transmittance Distribution function
/// implementation: Calculate the amount of light at a surface without
/// accounting for sub-surface scattering.

use crate::space::*;

mod fresnel;
mod specular;
mod diffuse;
mod microfacet;

pub use self::fresnel::Fresnel;

pub type MicrofacetDistribution = microfacet::Distribution;

bitflags! {
    pub struct BxDFType: u32 {
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
pub enum TransportMode { Radiance, Importance }

/// Details about the sample for a BxDF integral in some direction
pub struct BxDFSample {
    pub spectrum: Color,
    pub wi: Vector,
    pub pdf: f64,
    pub t: Option<BxDFType> // sampled type
}
impl BxDFSample {
    fn new(spectrum: Color, wi: Vector) -> BxDFSample {
        BxDFSample { spectrum, wi, pdf: 1.0, t: None }
    }
}

/// Specifications for different kinds of generic Bidirectional
/// Reflectance/Transmittance Distribution Functions, used in Material
/// definitions.
pub enum BxDF {

    /// With specular reflection. First parameter is reflection specturm and
    /// second is Fresnel light transport model.
    SpecularReflection(specular::Reflection),

    /// With specular transmission.
    SpecularTransmission(specular::Transmission),

    /// Combined specular reflection and transmission, used with Monte-Carlo
    /// integrator.
    Specular(specular::Combined),

    /// Diffuse reflection, using more physically accurate OrenNayar model
    /// compared to Lambertian.
    Diffuse(diffuse::OrenNayar),

    /// Microfacet reflection with Trowbridge-Reitz distribution implementation.
    MicrofacetReflection(microfacet::Reflection),

    /// Microfacet transmission with Trowbridge-Reitz distribution implementation.
    MicrofacetTransmission(microfacet::Transmission),

    /// Function with scaled (partial) contribution, given by the color.
    Scaled(Box<BxDF>, Color),
}

impl BxDF {
    pub fn specular_reflection(r: Color, fresnel: Fresnel) -> BxDF {
        let reflection = specular::Reflection::new(r, fresnel);
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

    pub fn microfacet_reflection(r: Color, fresnel: Fresnel, distribution: microfacet::Distribution) -> BxDF {
        let reflection = microfacet::Reflection::new(r, fresnel, distribution);
        BxDF::MicrofacetReflection(reflection)
    }

    pub fn microfacet_transmission(t: Color, eta_a: f64, eta_b: f64, mode: TransportMode, distribution: microfacet::Distribution) -> BxDF {
        let transmission =
            microfacet::Transmission::new(t, eta_a, eta_b, mode, distribution);
        BxDF::MicrofacetTransmission(transmission)
    }

    pub fn scaled(bxdf: BxDF, spectrum: Color) -> BxDF {
        BxDF::Scaled(Box::new(bxdf), spectrum)
    }

    /// Type
    pub fn t(&self) -> BxDFType {
        match self {
            BxDF::SpecularReflection(_) => BxDFType::REFLECTION | BxDFType::SPECULAR,
            BxDF::SpecularTransmission(_) => BxDFType::TRANSMISSION | BxDFType::SPECULAR,
            BxDF::Specular(_) => BxDFType::REFLECTION | BxDFType::TRANSMISSION | BxDFType::SPECULAR,
            BxDF::Diffuse(_) => BxDFType::REFLECTION | BxDFType::DIFFUSE,
            BxDF::MicrofacetReflection(_) => BxDFType::REFLECTION | BxDFType::GLOSSY,
            BxDF::MicrofacetTransmission(_) => BxDFType::TRANSMISSION | BxDFType::GLOSSY,
            BxDF::Scaled(bxdf, _) => bxdf.t(),
        }
    }

    /// Evaluate the distribution function for outgoing vector w0 and incident
    /// direction wi. Actual value, not a sample or estimate.
    pub fn f(&self, wo: &Vector, wi: &Vector) -> Color {
        match self {
            BxDF::Diffuse(d) => d.f(wo, wi),
            BxDF::MicrofacetReflection(r) => r.f(wo, wi),
            BxDF::MicrofacetTransmission(t) => t.f(wo, wi),
            BxDF::Scaled(bxdf, scale) => scale.mul_element_wise(bxdf.f(wo, wi)),
            _ => Color::zero(), // Specular has no scattering, only sampling
        }
    }

    /// BxDFSample the value of f - call this multiple times and average to
    /// approximate the evaluation of the BxDF integral (i.e., Monte Carlo
    /// Integration). Also returns the type of the resolved sample.
    pub fn sample_f(&self, wo: &Vector, sample: &Point2f) -> BxDFSample {
        match self {
            BxDF::SpecularReflection(r) => { return r.sample_f(wo, sample) },
            BxDF::SpecularTransmission(t) => { return t.sample_f(wo, sample) }
            // BxDF::Specular(s) => { return s.sample_f(wo, sample) }
            _ => {}
        }
        // TODO
        /*
        // Cosine-sample the hemisphere, flipping the direction if necessary
        let mut wi = cosine_sample_hemisphere(sample);
        if (wo.z < 0.0) { wi.z *= -1 };
        let spectrum = self.f(wo, &wi);
        let mut sample = BxDFSample::new(spectrum, wi);
        sample.pdf = if same_hemisphere(wo, wi) {
            abs_cos_theta(wi) * FRAC_1_PI
        } else { 0.0 };
        sample
        */
        BxDFSample::new(Color::zero(), Vector::zero())
    }

    /// Hemispherical-Directional Reflectance funtion gives total reflection in
    /// a given direction due to constant illumination over the hemisphere
    /// (which happens to also be equivalent to reflection in all directions
    /// based in light from a single incoming direction).
    pub fn rho_hd(&self, w0: &Vector, wi: &Vector, samples: &[Point2f]) -> Color {
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

    #[inline] pub fn reflect(w0: &Vector, n: &Vector) -> Vector {
        -1.0 * w0 + 2.0 * w0.dot(*n) * n
    }

    /// Refract the given incident direction vector into a medium with the
    /// refractive index `eta`. Returns `None` if the refraction fails due to Total
    /// Internal Reflection.
    #[inline] pub fn refract(wi: &Vector, n: &Normal, eta: f64) -> Option<Vector> {
        // Compute cos_theta_t w/ Snell's law
        let n = n.as_vec();
        let cos_theta_i = n.dot(*wi);
        let sin2_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0);
        let sin2_theta_t = eta * eta * sin2_theta_i;

        // Handle total internal reflection
        if sin2_theta_t >= 1.0 { return None }

        let cos_theta_t = (1.0 - sin2_theta_t).sqrt();
        Some(eta * -1.0 * wi + (eta * cos_theta_i - cos_theta_t) * n)
    }
}
