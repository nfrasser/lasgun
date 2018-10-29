use std::f64;
use crate::space::*;
use crate::ray::Ray;
use crate::primitive::Primitive;
use crate::interaction::SurfaceInteraction;

use super::{Light, LightSampleIterator};

/// A Point Light has no surface area an emits in all directions
/// These don't exist in real life but are a good approximation
#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
    pub falloff: [f64; 3]
}

impl PointLight {
    pub fn new(position: [f64; 3], intensity: [f64; 3], falloff: [f64; 3]) -> PointLight {
        PointLight {
            position: Point::new(position[0], position[1], position[2]),
            intensity: Color::new(intensity[0], intensity[1], intensity[2]),
            falloff
        }
    }
}

impl Light for PointLight {

    /// Returns the intersity of the light received at the given point.
    /// Equivalent to `I / f_att`, where `I` is intensity and `f_att` is
    /// attentuation based on distance (squared). e.g.,
    ///
    ///     // Calculate attenuation
    ///     let d = 42.0; // distance
    ///     let falloff = [1.0, 0.0, 0.0];
    ///     let f_att = falloff[0] + falloff[1]*d + falloff[2]*d*d;
    ///     println!("{}", f_att);
    ///
    fn sample(&self, root: &dyn Primitive, p: &Point) -> Option<PointLight> {
        let d = self.position - p; // direction from p to light
        let t = d.magnitude(); // distance to light in world coordinates

        // Move point slighly outside the surface of the intersecting primitive
        // accounts for floating point errors
        let p = p + (f64::EPSILON * (1 << 15) as f64) * d;

        // Create a shadow ray
        let ray = Ray::new(p, d, 0, false);

        // See if there's anything that intersects
        let mut interaction = SurfaceInteraction::none();
        root.intersect(&ray, &mut interaction);
        if interaction.exists() && interaction.t < t {
            None
        } else {
            Some(*self)
        }
    }

    fn iter_samples<'l, 's>(&'l self, root: &'s dyn Primitive, p: Point)
    -> LightSampleIterator<'l, 's> {
        // Point lights only require one sample
        LightSampleIterator::new(self, root, p, 1)
    }
}

unsafe impl Sync for PointLight {}
