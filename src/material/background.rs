use crate::space::*;

#[derive(Debug, Copy, Clone)]
pub struct Background {
    pub inner: Color,
    pub outer: Color,
    pub view: Vector,
    // FIXME: Convert to scale
    fov: f64
}

impl Background {
    pub fn radial(inner: Color, outer: Color, view: Vector, fov: f64) -> Background {
        Background { inner, outer, view: view.normalize(), fov }
    }

    pub fn solid(color: Color) -> Background {
        Background::radial(color, color, Vector::unit_z(), 1.0)
    }

    /// Compute the background colour based on the direction vector
    /// Assume d is normalized
    pub fn bg(&self, d: &Vector) -> Color {
        let mut t = 1.0 - self.view.dot(*d).abs();
        t *= 270.0/self.fov;
        t = t.min(1.0);

        Color {
            x: lerp(t, self.inner.x, self.outer.x),
            y: lerp(t, self.inner.y, self.outer.y),
            z: lerp(t, self.inner.z, self.outer.z)
        }
    }
}
