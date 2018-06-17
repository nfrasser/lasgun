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
        Direction { vec, inv: Vector::new(1.0/vec.x, 1.0/vec.y, 1.0/vec.z) }
    }
}

#[inline]
pub fn len(v: &Vector) -> f64 {
    let v2 = v.component_mul(v);
    (v2.x + v2.y + v2.z).sqrt()
}
