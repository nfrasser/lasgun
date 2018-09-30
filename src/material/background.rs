use space::*;
use scene::Scene;

#[derive(Debug)]
pub struct Background {
    color: Color
}

impl Background {
    pub fn new(color: Color) -> Background {
        Background { color }
    }

    pub fn black() -> Background {
        let black = Color::zeros();
        Background { color: black }
    }
}

impl super::Material for Background {
    fn color(&self, _q: &Point, _e: &Point, _n: &Normal, _scene: &Scene) -> Color {
        self.color
    }
}
