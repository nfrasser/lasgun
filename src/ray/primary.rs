use std::f64;

use crate::{
    space::*,
    primitive::Primitive,
    interaction::SurfaceInteraction,
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
    pub fn at(scene: &Scene, x: usize, y: usize) -> PrimaryRay {
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

            let mut interaction = SurfaceInteraction::default();
            root.intersect(&ray, &mut interaction);

            // Calculates the actual intersection point and normalizes.
            // Required before getting p(), d(), etc.
            interaction.commit(&ray);

            // Get the correct scene material
            let material = scene.material_or_background(&interaction.material);

            // Query the material for the color at the given point
            color += material.color(&interaction, root)
        }

        color * scene.supersampling.power
    }
}
