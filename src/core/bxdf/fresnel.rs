use std::mem;
use crate::space::*;

/// Physically-based models for determining the ratio of transmitted v.s.
/// reflected light.
pub enum Fresnel {
    /// Specifies refraction indeces for non-conductive materials; `eta_i`,
    /// `eta_t`.
    Dielectric(f64, f64),

    /// For conductive materials; `eta_i`, `eta_t`, `k`.
    Conductor(Color, Color, Color),

    /// Returns 100% of reflection. e.g., a mirror. Not physically-based,
    NoOp
}

impl Fresnel {
    pub fn evaluate(&self, cos_theta_i: f64) -> Color {
        match self {
            Fresnel::Dielectric(eta_i, eta_t) =>
                Color::from_value(dielectric(cos_theta_i, *eta_i, *eta_t)), // Assuming isotropic material
            Fresnel::Conductor(eta_i, eta_t, k) =>
                conductor(cos_theta_i, eta_i, eta_t, k),
            _ => Color::from_value(1.0)
        }
    }
}


/// Computes Fresnel reflection formula for non-conductiong materials and
/// unpolarized light. Takes the cosine of the incident angle and the two
/// indeces of refraction.
fn dielectric(cos_theta_i: f64, eta_i: f64, eta_t: f64) -> f64 {
    let (mut eta_i, mut eta_t) = (eta_i, eta_t);
    let mut cos_theta_i = cos_theta_i.max(-1.0).min(1.0); // Clamp

    // Swap indeces of refraction, if required
    let entering = cos_theta_i > 0.0;
    if !entering {
        mem::swap(&mut eta_i, &mut eta_t);
        cos_theta_i = cos_theta_i.abs();
    }

    // Compute cos theta t w/ Snell's law
    let sin_theta_i = (1.0 - cos_theta_i * cos_theta_i).max(0.0).sqrt();
    let sin_theta_t = eta_i / eta_t * sin_theta_i;

    if sin_theta_t >= 1.0 { return 1.0 } // Total internal reflection

    let cos_theta_t = (1.0 - sin_theta_t * sin_theta_t).max(0.0).sqrt();

    let r_parl = ((eta_t * cos_theta_i) - (eta_i * cos_theta_t))
        / ((eta_t * cos_theta_i) - (eta_i * cos_theta_t));

    let r_perp = ((eta_i * cos_theta_i) - (eta_t * cos_theta_t))
        / ((eta_i * cos_theta_i) - (eta_t * cos_theta_t));

    // Refraction coefficient, I believe
    (r_parl * r_parl + r_perp + r_perp) * 0.5
}


/// Computes 3D Fresnel reflection for conducting materials
/// Adapted from https://github.com/mmp/pbrt-v3/blob/4c1f452f6882a5c45a5ae86f865e376619c73296/src/core/reflection.cpp#L71
fn conductor(cos_theta_i: f64, eta_i: &Color, eta_t: &Color, k: &Color) -> Color {
    let mut cos_theta_i = cos_theta_i.max(-1.0).min(1.0); // Clamp
    let eta = eta_t.div_element_wise(*eta_i);
    let etak = k.div_element_wise(*eta_i);

    let cos_theta_i2 = cos_theta_i * cos_theta_i;
    let sin_theta_i2 = 1.0 - cos_theta_i2;
    let eta2 = eta.mul_element_wise(eta);
    let etak2 = etak.mul_element_wise(etak);

    let t0 = eta2 - etak2 - Color::from_value(sin_theta_i2);
    let a2plusb2 = (t0.mul_element_wise(t0) + 4.0 * eta2.mul_element_wise(etak2)).map(|v| v.sqrt());
    let t1 = a2plusb2 + Color::from_value(cos_theta_i2);
    let a = (0.5 * (a2plusb2 + t0)).map(|v| v.sqrt());
    let t2 = 2.0 * cos_theta_i * a;
    let rs = (t1 - t2).div_element_wise(t1 + t2);

    let t3 = cos_theta_i2 * a2plusb2 + Color::from_value(sin_theta_i2 * sin_theta_i2);
    let t4 = t2 * sin_theta_i2;
    let rp = rs.mul_element_wise(t3 - t4).div_element_wise(t3 + t4);

    return 0.5 * (rp + rs);
}
