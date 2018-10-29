use super::space::Point;
use super::primitive::Primitive;
use std::marker::Sync;

pub mod point;
pub use self::point::PointLight;

pub trait Light: Sync {
    /// Sample the light received by the given point in the scene. The returned
    /// point light is to be used in shading calculations. A None is returned if
    /// an internally-calculated PointLight sample is not visible from the given
    /// point. Depending on the Light implementation
    fn sample(&self, root: &dyn Primitive, p: &Point) -> Option<PointLight>;

    /// Create an iterator that yields point lights that are visible from the
    /// given point in the given scene. Most implementations return
    /// LightSampleIterator instances initialized as are required given the
    /// scene parameters for a nice rendering
    fn iter_samples<'l, 's>(&'l self, root: &'s dyn Primitive, p: Point) -> LightSampleIterator<'l, 's>;
}

/// An iteratator for conveniently looping through samples taken from a given
/// light that are visible from the given point. The number of iterations
/// depends on the type of light and the sampling settings on the scene
pub struct LightSampleIterator<'l, 's> {
    light: &'l dyn Light,
    root: &'s dyn Primitive,
    point: Point,
    /// Number of samples remaning
    remaining: usize,
}

impl<'l, 's> LightSampleIterator<'l, 's> {
    pub fn new(light: &'l dyn Light, root: &'s dyn Primitive, point: Point, samples: usize)
    -> LightSampleIterator<'l, 's> {
        LightSampleIterator {
            light, root, point, remaining: samples
        }
    }
}

impl<'l, 's> Iterator for LightSampleIterator<'l, 's> {
    type Item = PointLight;

    fn next(&mut self) -> Option<PointLight> {
        while self.remaining > 0 {
            self.remaining -= 1;
            if let Some(light) = self.light.sample(self.root, &self.point) {
                return Some(light)
            }
        }
        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.remaining))
    }
}
