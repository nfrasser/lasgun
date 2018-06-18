use space::{ Point, Color };
use scene::Scene;
use light::Light;

/**
    A Point Light has no surface area an emits in all directions
    These don't exist in real life but are a good approximation
*/
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
    /**
    Returns the intersity of the light received at the given point. Equivalent to `I / f_att`,
    where `I` is intensity and `f_att` is attentuation based on distance (squared).

        f_att = falloff[0] + falloff[1]*d + fallof[2]*d*d
    */
    fn sample(&self, _p: &Point, _scene: &Scene, cb: &Fn(&PointLight) -> Color) -> Color {
        // TODO: Use shadow rays to check for intersection
        cb(&self)
    }
}
