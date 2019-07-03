use crate::{core::bxdf::BxDF, space::*, interaction::{SurfaceInteraction, BSDF}};
use super::Material;

pub struct Background {
    pub inner: Color,
    pub outer: Color,
    view: Vector,
    fov: f64
}

impl Background {
    pub fn radial(inner: Color, outer: Color, view: Vector, fov: f64) -> Background {
        Background { inner, outer, view, fov }
    }

    pub fn solid(color: Color, view: Vector, fov: f64) -> Background {
        Background::radial(color, color, view, fov)
    }

    /// Compute the background colour based on the interaction
    pub fn li(&self, interaction: &SurfaceInteraction) -> Color {
        let d = interaction.d();
        let mut t = (1.0 - self.view.dot(d).abs())/(d.magnitude() * self.view.magnitude());
        t *= 270.0/self.fov;
        t = t.min(1.0);

        Color {
            x: lerp(t, self.inner.x, self.outer.x),
            y: lerp(t, self.inner.y, self.outer.y),
            z: lerp(t, self.inner.z, self.outer.z)
        }
    }
}

impl Material for Background {
    fn scattering(&self, interaction: &SurfaceInteraction) -> BSDF {
        BSDF::new(interaction, &[BxDF::Constant(self.li(interaction))])
    }
}
