use std::f64;
use crate::{
    space::*,
    ray::Ray,
    interaction::SurfaceInteraction,
    scene::{MaterialRef},
    primitive::Primitive,
    Accel
};

/// A refractive material leverages the properties of another material in the
/// scene but continues colour calculations through to the other side following
/// the point of intersection (recursively).
///
/// All refractive materials also have some reflection, based on their
/// refractive properties.
pub struct Refractive {
    base: MaterialRef,

    /// refractive index
    index: f64,

    /// Used to determine refraction contribution to material appearance (per
    /// world unit). Should generally be between zero and one, where one means
    /// fully opaque (no refraction contribution).
    opacity: f64
}


impl Refractive {
    pub fn new(base: MaterialRef, index: f64, opacity: f64) -> Refractive {
        Refractive { base, index, opacity }
    }

    /// Enter the material via the given interaction and return the discovered
    /// color. Counts reflected light when attempting to enter. Assumes
    /// interaction has recursion levels available.
    fn enter(&self, primitive: &dyn Primitive, from_n: f64, interaction: &SurfaceInteraction, root: &Accel) -> Color {
        let refract = Refract::at(interaction, from_n, self.index);
        if let None = refract {
            return root.scene.material(&self.base).unwrap()
                .color(primitive, interaction, root)
        };
        let refract = refract.unwrap();

        let mut tinteraction = SurfaceInteraction::default();
        let tprim = root.intersect(&refract.tray, &mut tinteraction).unwrap_or(root);
        tinteraction.commit(&refract.tray);

        let base = root.scene.material(&self.base).unwrap()
            .color(primitive, interaction, root);

        // Calculate the transmitted color
        let tcolor = if tprim as *const Primitive == primitive as *const Primitive {
            // Exit back into the previous medium
            self.exit(tprim, from_n, &tinteraction, root)
        } else {
            // Existed out into the real world, probably. Fails if primitives
            // are nested. In which case we're out of luck until a proper
            // integrator is implemented.
            root.scene.material_or_background(&tinteraction.material)
                .color(tprim, &tinteraction, root)

        };

        // Calculate the reflected color
        let mut rinteraction = SurfaceInteraction::default();
        let rprim = root.intersect(&refract.rray, &mut rinteraction).unwrap_or(root);
        rinteraction.commit(&refract.rray);
        let rcolor = if let Some(m) = rinteraction.material {
            root.scene.material(&m).unwrap().color(rprim, &rinteraction, root)
        } else {
            root.scene.ambient
        };

        // let base = root.scene.material(&self.base).unwrap()
        //     .color(primitive, interaction, root);

        // Return the weighed contribution of each component
        tcolor*refract.tcontrib + rcolor*refract.rcontrib
        // base*self.opacity + (tcolor*refract.tcontrib*(1.0 - self.opacity)) + rcolor*refract.rcontrib
        // base
    }

    // Exit from this material via the given interaction and return what colour
    // is discovered upon exiting. Counts reflected light when attempting to
    // exit (e.g., Total Internal Reflection). Assumes interaction has recursion
    // levels available.
    fn exit(&self, primitive: &dyn Primitive, to_n: f64, interaction: &SurfaceInteraction, root: &Accel) -> Color {
        let refract = Refract::at(interaction, self.index, to_n);
        if let None = refract {
            return root.scene.background().color(primitive, interaction, root)
        };
        let refract = refract.unwrap();

        // Calculate the transmitted color (now outside the material)
        let mut tinteraction = SurfaceInteraction::default();
        let tprim = root.intersect(&refract.tray, &mut tinteraction).unwrap_or(root);
        tinteraction.commit(&refract.tray);
        let tcolor = root.scene.material_or_background(&tinteraction.material)
            .color(tprim, &tinteraction, root);

        // Calculate the reflected color (e.g., total internal reflection)
        let mut rinteraction = SurfaceInteraction::default();
        let rprim = root.intersect(&refract.rray, &mut rinteraction).unwrap_or(root);
        rinteraction.commit(&refract.tray);

        let rcolor = if rprim as *const Primitive == primitive as *const Primitive {
            // Enter back into the previous medium
            self.enter(rprim, to_n, &rinteraction, root)
        } else {
            // Existed out into the real world, probably. Fails if primitives
            // are nested. In which case we're out of luck until a proper
            // integrator is implemented.
            root.scene.material_or_background(&rinteraction.material)
                .color(rprim, &rinteraction, root)
        };

        // Return the weighed contribution of each component Inside a material,
        // meaning all the lights aren't visible (until we get sampling) so
        // don't count that
        tcolor*refract.tcontrib + rcolor*refract.rcontrib
    }
}

impl super::Material for Refractive {
    fn color(&self,
        primitive: &dyn Primitive,
        interaction: &SurfaceInteraction,
        root: &Accel
    ) -> Color {
        self.enter(primitive, 1.0, interaction, root)
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
    pub fn at(interaction: &SurfaceInteraction, n1: f64, n2: f64) -> Option<Refract> {
        if interaction.level() == 0 { return None }
        let one = 1.0;
        let i = interaction.d(); // incident vector
        let n = interaction.n.to_vec();
        let p = interaction.p();
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
        let eps = f64::EPSILON * 4294967296.0;
        let rray = Ray::reflect(p + (n*eps), r, interaction.level()).unwrap();
        let tray = Ray::refract(p - (n*eps), t, interaction.level()).unwrap();

        // Total internal reflection?
        let tir = sin2_theta_t > one;

        // Reflection opacity
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

        // Refraction opacity
        let tcontrib = one - rcontrib;

        Some(Refract { rray, tray, rcontrib, tcontrib })
    }

    // Returns true if no light is refracted, just reflected back toward the
    // inside instead.
    // pub fn has_refraction(&self) -> bool {
    //     self.tcontrib == N::zero()
    // }
}
