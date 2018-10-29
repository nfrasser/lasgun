use std::f64;
use crate::{
    space::*,
    ray::Ray,
    interaction::SurfaceInteraction,
    scene::{Scene, MaterialRef},
    primitive::Primitive,
};

/// A refractive material leverages the properties of another material in the
/// scene but continues colour calculations through to the other side following
/// the point of intersection (recursively)
pub struct Refractive {
    base: MaterialRef,

    /// refractive index
    index: f64,

    /// Reflection/refraction contribution to material appearance. Between zero
    /// and one. Higher numbers means less opaque materials.
    contrib: f64
}

impl Refractive {
    pub fn new(base: MaterialRef, index: f64, contrib: f64) -> Refractive {
        Refractive { base, index, contrib }
    }
}

impl super::Material for Refractive {
    fn color(&self, ray: &Ray, interaction: &SurfaceInteraction, scene: &Scene, root: &dyn Primitive)
    -> Color {
        let (n1, n2) = if ray.medium {
            (self.index, 0.0)
        } else {
            (0.0, self.index)
        };

        // Base color
        let base = if let Some(material) = scene.material(&self.base) {
            material.color(ray, interaction, scene, root)
        } else {
            Color::zero()
        };

        // Recursive color
        let recursive = if let Some(refract) = Refract::ray(ray, &interaction.p, &interaction.n, n1, n2) {
            let mut rinteraction = SurfaceInteraction::none();
            root.intersect(&refract.rray, &mut rinteraction);
            let rcolor = if rinteraction.exists() {
                rinteraction.p = refract.rray.origin + refract.rray.d*rinteraction.t;
                let material = scene.material(&rinteraction.material.unwrap()).unwrap();
                material.color(&refract.rray, &rinteraction, scene, root)
            } else {
                Color::zero()
            };

            let mut tinteraction = SurfaceInteraction::none();
            root.intersect(&refract.tray, &mut tinteraction);
            let tcolor = if tinteraction.exists() {
                tinteraction.p = refract.tray.origin + refract.tray.d*tinteraction.t;
                let material = scene.material(&tinteraction.material.unwrap()).unwrap();
                material.color(&refract.tray, &tinteraction, scene, root)
            } else {
                Color::zero()
            };

            (rcolor * refract.rcontrib) + (tcolor * refract.tcontrib)
        } else {
            Color::zero()
        };

        (recursive * self.contrib) + base
    }
}

/// Description of a ray refractin
pub struct Refract {
    /// Reflected ray
    pub rray: Ray,
    /// Transmitted ray
    pub tray: Ray,
    /// Contribution of the reflected ray [0, 1]
    pub rcontrib: f64,
    /// Contribution of the refracted ray [0, 1]
    pub tcontrib: f64,
}

impl Refract {
    /// Try a refraction of the ray through a medium defined by the given
    /// reflective indeces. n1 is the refractive index of the current material
    /// and n1 is the refractive index of the transmission material.
    ///
    /// Returns None if the ray has reached its maximum recursion level
    pub fn ray(ray: &Ray, p: &Point, n: &Normal, n1: f64, n2: f64) -> Option<Refract> {
        if ray.level == 0 { return None };
        let one = 1.0;
        let i = ray.d.normalize(); // incident vector
        let n = n.as_vec().normalize();
        let cos_theta_i = i.dot(n);

        // Reflection direction
        let r = -n * (cos_theta_i + cos_theta_i) + i;

        let n1_div_n2 = n1 / n2;

        // Sin squared of the transmission angle
        let sin2_theta_t = (n1_div_n2 * n1_div_n2) * (one - cos_theta_i * cos_theta_i);

        // Transmission direction
        let t = i*n1_div_n2 + n*(n1_div_n2*cos_theta_i - (one - sin2_theta_t).sqrt());

        // Approximate reflection with Schlick’s method
        let r_0_sqrt = (n1 - n2)/(n1 + n2);
        let r_0 = r_0_sqrt * r_0_sqrt;

        // Calculate error epsilon by which to multiply normal to sufficiently
        // push it away from the surface and avoid floaing point errors.
        let eps = f64::EPSILON * 32.0;
        let rray = Ray::reflect(ray, p + (n*eps), r).unwrap();
        let tray = Ray::refract(ray, p - (n*eps), t).unwrap();

        // Total internal reflection?
        let tir = sin2_theta_t > one;

        // Reflection contrib
        let rcontrib = if n1 <= n2 {
            let cos_theta_i_5 = (0..5).fold(one, |pow, _| pow*cos_theta_i); // power of 5
            r_0 + (one - r_0)*cos_theta_i_5
        } else if n1 > n2 && !tir {
            let cos_theta_t = (one - sin2_theta_t).sqrt();
            let cos_theta_t_5 = (0..5).fold(one, |pow, _| pow*cos_theta_t); // power of 5
            r_0 + (one - r_0)*cos_theta_t_5
        } else {
            one
        };

        // Refraction contrib
        let tcontrib = one - rcontrib;

        Some(Refract { rray, tray, rcontrib, tcontrib })
    }

    // Returns true if no light is refracted, just reflected back toward the
    // inside instead.
    // pub fn has_refraction(&self) -> bool {
    //     self.tcontrib == N::zero()
    // }
}
