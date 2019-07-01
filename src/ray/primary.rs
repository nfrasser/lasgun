use std::f64;

use crate::{
    space::*,
    core::bxdf,
    core::bxdf::BxDFType,
    primitive::Primitive,
    interaction::{BSDF, SurfaceInteraction},
    scene::Scene,
    Accel,
};

use super::Ray;

/**
The initial ray that gets cast from the camera to scene.
The resulting colour form the `cast` function will be a pixel in the resulting image.
*/
#[derive(Debug, Copy, Clone)]
pub struct PrimaryRay {
    pub origin: Point,
    pub d: Vector // not normalized
}

impl PrimaryRay {
    /**
    Create a new primary ray from an origin point and a vector that spans from the origin
    to the focal plane
    */
    pub fn new(origin: Point, d: Vector) -> PrimaryRay {
        PrimaryRay { origin, d }
    }

    /// Create a new primary ray casting into the given scene at film
    /// coordinates x/y
    pub fn at(scene: &Scene, x: u32, y: u32) -> PrimaryRay {
        let (width, height) = (scene.options.width, scene.options.height);
        let up = scene.up;
        let aux = scene.aux;
        let sample_distance = scene.pixel_radius * 2.0;

        let (x, y) = (x as f64, y as f64);

        // Calculate offsets distances from the view vector
        let hoffset = (x - ((width as f64 - 1.0) * 0.5)) * sample_distance;
        let voffset = ((height as f64 - 1.0) * 0.5 - y) * sample_distance;

        // The direction in which this ray travels
        let d = scene.view + (voffset * up) + (hoffset * aux);

        PrimaryRay::new(scene.eye, d)
    }

    /// Takes the scene, the scene's root node, and the background color
    pub fn cast(&self, root: &Accel) -> Color {
        let scene = root.scene;
        let dim = scene.supersampling.dim as i32;
        let mut color = Color::zero();

        for i in 0..scene.supersampling.count as i32 {
            // Calculate offset from the origin as factors of the supersampling radius
            // x = {0} => {0}
            // x = {0, 1} => {-1, 1}
            // x = {0, 1, 2} => {-2, 0, 2}
            // x = {0, 1, 2, 3} => {-3, 1, 1, 3}
            let xoffset = i % dim * 2 - dim + 1;
            let yoffset = i / dim * 2 - dim + 1;
            let auxoffset = scene.supersampling.radius * xoffset as f64;
            let upoffset = scene.supersampling.radius * yoffset as f64;

            // New point at which the ray intersects the focal plane given this direction
            let d = self.d + (upoffset * scene.up) + (auxoffset * scene.aux);
            let ray = Ray::new(self.origin, d);
            color += self.li(root, &ray, 0);
        }

        color * scene.supersampling.power
    }

    /// Whitted colorization strategy
    fn li(&self, root: &Accel, ray: &Ray, depth: u32) -> Color {
        let mut interaction = SurfaceInteraction::default();
        root.intersect(&ray, &mut interaction);

        // Calculates the actual intersection point and normalizes.
        // Required before getting p(), d(), etc.
        interaction.commit(&ray);

        // Return background if an intersection wasn't found
        if interaction.material.is_none() {
            return root.scene.background.li(&interaction, root)
        }
        let material = root.scene.material_or_background(&interaction.material);

        // Compute emitted and reflected light at intersection point
        // Initialize common vars
        let n = interaction.n.to_vec();
        let wo = -interaction.d(); // Outgoing direction
        let p = interaction.p();

        // Compute scattering functions
        let bsdf = material.scattering(&interaction, root);

        // Add contribution of each light source
        // For each scene light, sample point lights from it
        root.scene.lights().iter().fold(Color::zero(), |output, light| {
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
        }) + if depth + 1 < MAX_DEPTH {
            // Add reflection/transmission contribution
            self.specular_reflect(root, ray, &interaction, &bsdf, depth)
            // + self.specular_transmit(root, ray, &interaction, &bsdf, depth)
        } else {
            Color::zero()
        } + root.scene.ambient
    }

    fn specular_reflect(&self, root: &Accel, ray: &Ray, interaction: &SurfaceInteraction, bsdf: &BSDF, depth: u32) -> Color {
        // Compute specular reflection direction wi and BSDF value
        let wo = -interaction.d();
        let flags = BxDFType::REFLECTION | BxDFType::SPECULAR;

        // TODO: Use actual sample point instead of (0.5, 0.5)
        let sample = bsdf.sample_f(&wo, &Point2f::new(0.5, 0.5), flags);

        // Return contribution of specular reflection
        let ns = interaction.n.0;
        if sample.pdf > 0.0
        && sample.spectrum != Color::zero()
        && sample.wi.dot(ns).abs() > 0.0 {
            // Compute ray for specular reflection
            let wr = bxdf::util::reflect(&wo, &ns);
            let r = Ray::new(interaction.p(), wr);
            sample.spectrum.mul_element_wise(self.li(root, &r, depth - 1))
        } else {
            Color::zero()
        }
    }
}

// TODO: Parametrize
const MAX_DEPTH: u32 = 3;
