use space::{Point, Color};
use scene::Scene;

pub trait Light {
    /**
        Get the intensity of the light received by the given point
        in the scene.

        e.g., For point lights, this is either
        (A) the color, or
        (B) the Zero vector when something blocks the light

        In the future, area lights will return an intensity based on a sample
        from the point to the light
    */
    fn intensity(&self, p: &Point, scene: &Scene) -> Color;
}

pub mod point;
