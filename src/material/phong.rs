use crate::space::*;
use crate::scene::Scene;

// Phong-lighted material
pub struct Phong {
    kd: Vector,
    ks: Vector,
    shininess: i32
}

impl Phong {
    pub fn new(kd: [f64; 3], ks: [f64; 3], shininess: i32) -> Phong {
        Phong {
            kd: Vector::new(kd[0], kd[1], kd[2]),
            ks: Vector::new(ks[0], ks[1], ks[2]),
            shininess
        }
    }
}

impl super::Material for Phong {
    fn color(&self,
        q: &Point, eye: &Point, normal: &Normal,
        scene: &Scene
    ) -> Color {
        let n = normal.as_ref().normalize();
        let v: Vector = (eye - q).normalize();
        let ambient = Color::new(
            scene.options.ambient[0],
            scene.options.ambient[1],
            scene.options.ambient[2]
        );

        // start with ambient lighting
        let output = self.kd.mul_element_wise(ambient);

        // For each scene light, sample point lights from it
        scene.lights.iter().fold(output, |output, scene_light| {
            // For each sampled point light, add its contribution to the the
            // final colour output
            scene_light.iter_samples(scene, *q).fold(output, |output, light| {
                // vector to light and its length (distance to the light from q)
                let l = light.position - q;
                let d = l.magnitude();
                let l = l.normalize();
                let n_dot_l = n.dot(l);

                // Vector at the angle of reflection
                let r: Vector = 2.0*n_dot_l*n - l;
                let r_dot_v = r.dot(v);

                // Light attenuation over distance used to compute energy received at q
                let f_att = light.falloff[0] + light.falloff[1]*d + light.falloff[2]*d*d;
                let e = light.intensity / f_att;

                // Use material properties to determine color at given pixel
                // as if this is the only light in the scene
                self.kd.mul_element_wise(e)*n_dot_l.max(0.0) +
                output + self.ks.mul_element_wise(e)*r_dot_v.max(0.0).powi(self.shininess)
            })
        })

    }
}
