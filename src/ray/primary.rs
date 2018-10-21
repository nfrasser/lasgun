use std::f64;

use crate::{
    space::*,
    primitive::Primitive,
    material::Material,
    interaction::SurfaceInteraction,
    scene::Scene,
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

    /// Takes the scene, the scene's root node, and the background color
    pub fn cast(&self, scene: &Scene, root: &impl Primitive, bg: Color) -> Color {
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

            let mut interaction = SurfaceInteraction::none();
            root.intersect(&ray, &mut interaction);
            if !interaction.exists() {
                color += bg;
                continue
            };

            // Try getting the material
            let material: &dyn Material;
            if let Some(mref) = interaction.material {
                if let Some(m) = scene.material(&mref) { material = m }
                else { color += bg; continue }
            } else { color += bg; continue }

            // The vector spanning from the eye to the point of intersection
            // eye + direction = point of intersection
            let normal = &interaction.n;

            // Add a small fraction of the normal to avoid speckling due to
            // floating point errors (the calculated point ends up inside the
            // geometric primitive).
            interaction.p = ray.origin
                + interaction.t * ray.d
                + (f64::EPSILON * 32.0) * normal.as_vec();

            // Query the material for the color at the given point
            color += material.color(&interaction.p, &ray.origin, normal, scene, root)
        }

        color * scene.supersampling.power
    }
}
