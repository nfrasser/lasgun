use crate::{space::*, primitive::Primitive, interaction::SurfaceInteraction, Accel};
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
    fn color(&self, _: &dyn Primitive, interaction: &SurfaceInteraction, root: &Accel) -> Color {
        let view = root.scene.view.normalize();
        let d = interaction.d();
        let mut t = (1.0 - view.dot(d).abs())/(d.magnitude() * view.magnitude());
        t *= 270.0/root.scene.options.fov;
        t = t.min(1.0);

        Color {
            x: lerp(t, self.inner.x, self.outer.x),
            y: lerp(t, self.inner.y, self.outer.y),
            z: lerp(t, self.inner.z, self.outer.z)
        }
    }
}
