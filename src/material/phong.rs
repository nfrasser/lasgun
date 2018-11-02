use std::f64;
use crate::space::*;
use crate::{interaction::SurfaceInteraction, Accel};

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
fn color(&self, interaction: &SurfaceInteraction, root: &Accel) -> Color {
        let n = interaction.n.as_vec().normalize();
        let v: Vector = interaction.d().normalize();

        // Add a small fraction of the normal to avoid speckling due to floating
        // point errors (the calculated point ends up inside the geometric
        // primitive).
        let q = interaction.p() + (f64::EPSILON * 32.0) * n;

        let ambient = Color::new(
            root.scene.options.ambient[0],
            root.scene.options.ambient[1],
            root.scene.options.ambient[2]);

        // start with ambient lighting
        let output = self.kd.mul_element_wise(ambient);

        // For each scene light, sample point lights from it
        let result = root.scene.lights().iter().fold(output, |output, light| {
            // For each sampled point light, add its contribution to the the
            // final colour output
            light.iter_samples(root, q).fold(output, |output, plight| {
                // vector to plight and its length (distance to the plight from q)
                let l = plight.position - q;
                let d = l.magnitude();
                let l = l.normalize();
                let n_dot_l = n.dot(l);

                // Vector at the angle of reflection
                let r: Vector = 2.0*n_dot_l*n - l;
                let r_dot_v = r.dot(v);

                // Light attenuation over distance used to compute energy received at q
                let f_att = plight.falloff[0] + plight.falloff[1]*d + plight.falloff[2]*d*d;
                let e = plight.intensity / f_att;

                // Use material properties to determine color at given pixel
                // as if this is the only light in the scene
                self.kd.mul_element_wise(e)*n_dot_l.max(0.0) +
                output + self.ks.mul_element_wise(e)*r_dot_v.max(0.0).powi(self.shininess)
            })
        });

        result
        // Color::new(result.x.min(1.0), result.y.min(1.0), result.z.min(1.0))
    }
}

unsafe impl Sync for Phong {}
