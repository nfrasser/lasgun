use space::{ Point, Color };
use scene::Scene;
use light::Light;

/**
    A Point Light receives
*/
pub struct PointLight {
    position: Point,
    color: Color,
    falloff: (f64, f64, f64)
}

impl PointLight {
    pub fn new(position: Point, color: Color, falloff: (f64, f64, f64)) -> PointLight {
        PointLight { position, color, falloff }
    }
}

impl Light for PointLight {
    fn intensity(&self, p: &Point, scene: &Scene) -> Color {
        // TODO
        self.color.clone()
    }
}
