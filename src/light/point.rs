use space::{ Point, Color };
use scene::Scene;
use light::Light;

/**
    A Point Light has no surface area an emits in all directions
    These don't exist in real life but are a good approximation
*/
pub struct PointLight {
    position: Point,
    color: Color,
    falloff: [f64; 3]
}

impl PointLight {
    pub fn new(position: [f64; 3], color: [f64; 3], falloff: [f64; 3]) -> PointLight {
        PointLight {
            position: Point::new(position[0], position[1], position[2]),
            color: Color::new(color[0], color[1], color[2]),
            falloff
        }
    }
}

impl Light for PointLight {
    fn intensity(&self, p: &Point, scene: &Scene) -> Color {
        // TODO
        self.color.clone()
    }
}
