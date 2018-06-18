use space::*;
use shape::Shape;
use shape::Intersection;

/**
aka "Box", aka "Rectangular prism"
*/
pub struct Cuboid {
    pub bounds: (Point, Point)
}

impl Cuboid {
    pub fn new(bound0: Point, bound1: Point) -> Cuboid {
        Cuboid { bounds: (bound0, bound1) }
    }

    pub fn cube(origin: Point, dim: f64) -> Cuboid {
        Cuboid { bounds: (origin, origin + Vector::repeat(dim)) }
    }
}

impl Shape for Cuboid {
    fn intersect(&self, _e: &Point, _d: &Direction) -> Intersection {
        let (_b0, _b1) = (&self.bounds.0, &self.bounds.1);
        Intersection::none()
    }
}
