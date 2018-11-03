use crate::{space::*, interaction::SurfaceInteraction, Accel};
use super::Material;

pub struct Background {
    inner: Color,
    outer: Color,
}

impl Background {
    pub fn radial(inner: Color, outer: Color) -> Background {
        Background { inner, outer }
    }

    pub fn solid(color: Color) -> Background {
        Background::radial(color, color)
    }
}

impl Material for Background {
    fn color(&self, interaction: &SurfaceInteraction, root: &Accel) -> Color {
        let view = root.scene.view;
        let d = interaction.d();
        let t = ((1.0 - (view.dot(d)/(d.magnitude() * view.magnitude())).abs().sqrt()) * (360.0/root.scene.options.fov)).min(1.0);

        Color {
            x: lerp(t, self.inner.x, self.outer.x),
            y: lerp(t, self.inner.y, self.outer.y),
            z: lerp(t, self.inner.z, self.outer.z)
        }
    }
}
