use na::Vector3;
use space::{ Vector, Point, Color };
use scene::Scene;

use material::Material;

// Phong-lighted material
pub struct Phong {
    kd: Vector3<f64>,
    ks: Vector3<f64>,
    shininess: f64
}

impl Phong {
    pub fn new(kd: [f64; 3], ks: [f64; 3], shininess: f64) -> Phong {
        Phong {
            kd: Vector3::new(kd[0], kd[1], kd[2]),
            ks: Vector3::new(ks[0], ks[1], ks[2]),
            shininess
        }
    }
}

impl Material for Phong {
    fn color(&self,
        q: &Point, e: &Point, n: &Vector,
        scene: &Scene
    ) -> Color {
        // TODO
        Color::new(1.0, 1.0, 1.0)
    }
}
