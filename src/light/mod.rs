use space::{Point, Color};
use scene::Scene;

pub mod point;
pub use self::point::PointLight;

pub trait Light {
    /**
    Sample the light received by the given point in the scene.

    The given callback will be called a number of times with a point light
    representing a point on the light and subsequently how much energy it emits.

    A point light is to be used in shading calculations.

    Point lights passed to the callback are always visible from the given point
    */
    fn sample(&self, p: &Point, scene: &Scene,
        callback: &Fn(&PointLight) -> Color
    ) -> Color;
}
