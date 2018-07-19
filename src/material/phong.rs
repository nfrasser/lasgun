use na::{Unit, Vector3};
use space;
use space::{ Vector, Point, Color };
use scene::Scene;

// Phong-lighted material
pub struct Phong {
    kd: Vector3<f64>,
    ks: Vector3<f64>,
    shininess: i32
}

impl Phong {
    pub fn new(kd: [f64; 3], ks: [f64; 3], shininess: i32) -> Phong {
        Phong {
            kd: Vector3::new(kd[0], kd[1], kd[2]),
            ks: Vector3::new(ks[0], ks[1], ks[2]),
            shininess
        }
    }
}

impl super::Material for Phong {
    fn color(&self,
        q: &Point, eye: &Point, normal: &Unit<Vector>,
        scene: &Scene
    ) -> Color {
        let n: &Vector = normal.as_ref();
        let v = (eye - q).normalize();
        let mut output = scene.options.ambient.component_mul(&self.kd); // start with ambient lighting

        for scene_light in scene.options.lights.iter() {

            // Sample point lights
            output += scene_light.sample(q, scene, &|light| {
                // vector to light and its length (distance to the light from q)
                let l = light.position - q;
                let d = space::len(&l);
                let l = l.normalize();
                let n_dot_l = n.dot(&l);

                // Vector at the angle of reflection
                let r = 2.0*n_dot_l*n - l;
                let r_dot_v = r.dot(&v);

                // Light attenuation over distance used to compute energy received at q
                let f_att = light.falloff[0] + light.falloff[1]*d + light.falloff[2]*d*d;
                let e = light.intensity / f_att;

                // Use material properties to determine color at given pixel
                // as if this is the only light in the scene
                self.kd.component_mul(&e)*n_dot_l.max(0.0) +
                self.ks.component_mul(&e)*r_dot_v.max(0.0).powi(self.shininess)
            })
        }

        output
    }
}
