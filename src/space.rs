// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
use na;

pub type Vector = na::Vector3<f64>;
pub type Point = na::Point3<f64>;
pub type Color = na::Vector3<f64>;

/**
    The direction of a ray with a pre-computed inverse for box intersections
*/
pub struct Direction {
    pub vec: Vector,
    pub inv: Vector
}

impl Direction {
    pub fn new(vec: Vector) -> Direction {
        Direction { vec, inv: na::one()/vec }
    }
}
