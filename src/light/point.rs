use std::f64;
use crate::{
    space::*,
    primitive::Primitive,
    interaction::RayIntersection,
    Accel
};

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
            position: position.into(),
            intensity: intensity.into(),
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
    fn sample(&self, root: &Accel, p: &Point) -> Option<PointLight> {
        let d = self.position - p; // direction from p to light
        let ray = Ray::new(*p, d);

        // See if there's anything that intersects
        let mut isect = RayIntersection::default();
        root.intersect(&ray, &mut isect);
        if isect.t < 1.0 {
            None
        } else {
            Some(*self)
        }
    }

    fn iter_samples<'l, 's>(&'l self, root: &'s Accel<'s>, p: Point)
    -> LightSampleIterator<'l, 's> {
        // Point lights only require one sample
        LightSampleIterator::new(self, root, p, 1)
    }
}
