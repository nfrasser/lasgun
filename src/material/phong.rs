use na::Vector3;
use space::{ Vector, Point, Color };
use scene::Scene;

use material::Material;

// Phong-lighted material
pub struct PhongMaterial {
    kd: Vector3<f64>,
    ks: Vector3<f64>,
    shininess: f64
}

impl PhongMaterial {
    pub fn new(kd: Vector3<f64>, ks: Vector3<f64>, shininess: f64) -> PhongMaterial {
        PhongMaterial { kd, ks, shininess }
    }
}

impl Material for PhongMaterial {
    fn color(&self,
        q: &Point, e: &Point, n: &Vector,
        scene: &Scene
    ) -> Color {
        // TODO
        Color::new(1.0, 1.0, 1.0)
    }
}
