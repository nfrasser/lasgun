// use crate::space::*;

/*
enum Interaction {
    Surface(SurfaceInteraction)
    // TODO:
    // Medium(medium::MediumInteraction)
}

impl Interaction {
    fn p(&self) -> Point {
        match self { Interaction::Surface(interaction) => interaction.p }
    }

    fn n(&self) -> Normal {
        match self { Interaction::Surface(interaction) => interaction.n }
    }
}
*/

pub mod surface;
pub type SurfaceInteraction = surface::SurfaceInteraction<f64>;
