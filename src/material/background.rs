use std::f64;
use crate::space::*;

#[derive(Debug, Copy, Clone)]
pub struct Background {
    pub inner: Color,
    pub outer: Color,
    scale: f64
}

impl Background {
    /// Create a radial background with the given gradient scale, where scale
    /// ranges from 0 to 1. It is used to determine the extent of the gradient
    /// projected onto the "front and back" of the world sphere.
    pub fn radial(inner: Color, outer: Color, scale: f64) -> Background {
        Background { inner, outer, scale }
    }

    pub fn solid(color: Color) -> Background {
        Background::radial(color, color, 1.0)
    }

    /// Compute the background colour based on the direction vector
    /// Assume d is normalized
    pub fn bg(&self, d: &Vector) -> Color {
        // Even gradient based on the equation of a unit circle y = sqrt(1 - x^2)
        // Modified by scale [0, 1].
        let t = ((1. - Vector::unit_z().dot(*d).abs().powf(2.)).sqrt() / self.scale).min(1.);
        Color {
            x: lerp(t, self.inner.x, self.outer.x),
            y: lerp(t, self.inner.y, self.outer.y),
            z: lerp(t, self.inner.z, self.outer.z)
        }
    }
}
