use std::f64;

use crate::{
    space::*,
    core::bxdf,
    core::bxdf::BxDFType,
    primitive::Primitive,
    interaction::{BSDF, SurfaceInteraction, RayIntersection},
    Accel,
};

/**
 * Integrate the given sample rays for a single pixel, with each ray contributing
 * weight to the final image.
 */
pub fn integrate(root: &Accel, samples: &[Ray], weight: f64) -> Color {
    let mut color = Color::zero();
    for ray in samples { color += li(root, ray, 0) }
    color * weight
}

/// Whitted colorization strategy
fn li(root: &Accel, ray: &Ray, depth: u32) -> Color {
    let mut isect = RayIntersection::default();
    let shape = root.intersect(&ray, &mut isect);
    if shape.is_none() {
        return root.scene.background.bg(&ray.d.normalize())
    }
    let shape = shape.unwrap();
    let material = shape.material().unwrap_or(isect.material);

    // Calculates the actual intersection point and normalizes.
    // Required before getting p(), d(), etc.
    let interaction = SurfaceInteraction::from(ray, &isect);

    // Compute emitted and reflected light at intersection point
    // Initialize common vars
    let n = interaction.ns.0; // Geometric shading normal vector
    let wo = interaction.wo; // Outgoing direction
    let p = interaction.p + interaction.p_err;

    // Compute scattering functions
    let bsdf = material.scattering(&interaction);

    // Add contribution of each light source
    // For each scene light, sample point lights from it
    let output = root.scene.lights().iter().fold(Color::zero(), |output, light| {
        // For each sampled point light, add its contribution to the the
        // final colour output
        light.iter_samples(root, p).fold(output, |output, light| {

            // vector to light and its length (distance to the light from q)
            let wi = light.position - p;
            let d = wi.magnitude();

            // Light attenuation over distance used to compute energy received at p
            let f_att = light.falloff[0] + light.falloff[1]*d + light.falloff[2]*d*d;
            if f_att == 0.0 { return output }; // No contribution

            let wi = wi.normalize();
            let wi_dot_n = wi.dot(n);

            let f = bsdf.f(&wo, &wi);

            output + ((f64::consts::PI * light.intensity).mul_element_wise(f) * wi_dot_n / f_att)
        })
    }) + root.scene.ambient.mul_element_wise(bsdf.f(&wo, &n));

    let (refracted, reflected) = if depth < root.scene.recursion {
        // Add reflection/transmission contribution
        (
            specular_transmit(root, &interaction, &bsdf, depth),
            specular_reflect(root, &interaction, &bsdf, depth)
        )
    } else {
        (Color::zero(), Color::zero())
    };

    output + reflected + refracted
}

fn specular_reflect(root: &Accel, interaction: &SurfaceInteraction, bsdf: &BSDF, depth: u32) -> Color {
    // Compute specular reflection direction wi and BSDF value
    let wo = interaction.wo;
    let flags = BxDFType::REFLECTION | BxDFType::SPECULAR;

    // TODO: Use actual sample point instead of (0.5, 0.5)
    let sample = bsdf.sample_f(&wo, &Point2f::new(0.5, 0.5), flags);

    // Return contribution of specular reflection
    let ns = interaction.ns.0;

    // Zero checks to avoid unnecessary computation
    if sample.pdf <= 0.0
    || sample.spectrum == Color::zero()
    || sample.wi.dot(ns) <= 0.0
    { return Color::zero() };

    // Compute ray for specular reflection
    let wr = bxdf::util::reflect(&wo, &ns);
    let r = Ray::new(interaction.p + interaction.p_err, wr);
    let li = li(root, &r, depth + 1);
    let output = sample.spectrum.mul_element_wise(li);

    output
}

fn specular_transmit(root: &Accel, interaction: &SurfaceInteraction, bsdf: &BSDF, depth: u32) -> Color {
    // Compute specular reflection direction wi and BSDF value
    let wo = interaction.wo;
    let flags = BxDFType::TRANSMISSION | BxDFType::SPECULAR;

    // TODO: Use actual sample point instead of (0.5, 0.5)
    let sample = bsdf.sample_f(&wo, &Point2f::new(0.5, 0.5), flags);
    let (spectrum, wi, pdf) = (sample.spectrum, sample.wi, sample.pdf);

    let ns = interaction.ns.0;

    // Zero checks to avoid unnecessary computation
    if pdf <= 0.0
    || spectrum == Color::zero()
    || wi.dot(ns).abs() == 0.0 {
        return Color::zero()
    }

    // Compute ray for specular refraction
    let r = Ray::new(interaction.p - interaction.p_err, wi);
    let li = li(root, &r, depth + 1);
    let output = spectrum.mul_element_wise(li) * wi.dot(ns).abs() / sample.pdf;

    output
}
