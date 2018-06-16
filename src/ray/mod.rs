use space::*;
use scene::Scene;

/**
    At its most basic, a ray casts itself into a scene
    and returns a color.
*/
pub trait Ray {
    fn cast(&self, scene: &Scene) -> Color;
}

pub mod primary;
