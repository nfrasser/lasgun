use std::f64;
use space::{ Point, Color, Direction };
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
    fn sample(&self, p: &Point, scene: &Scene, cb: &Fn(&PointLight) -> Color) -> Color {
        let d = self.position - p; // direction from p to light

        // Move point slighly outside the surface of the intersecting primitive
        // accounts for floating point erros
        let p = p + (f64::EPSILON * (1 << 15) as f64) * d;
        let direction = Direction::new(d);

        // See if there's anything that intersects
        let (intersection, _) = scene.intersect(&p, &direction);

        if intersection.exists() && intersection.t < 1.0 {
            // Intersection before the light, makes no contribution
            Color::zeros()
        } else {
            // Callback calculates the colour contribution to the surface by this light
            cb(&self)
        }
    }
}
