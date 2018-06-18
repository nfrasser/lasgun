// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
use na;

pub type Point = na::Point3<f64>;
pub type Vector = na::Vector3<f64>;
pub type Color = na::Vector3<f64>;

/**
    The direction of a ray with a pre-computed inverse for box intersections
*/
pub struct Direction {
    pub d: Vector,
    pub inv: Vector
}

impl Direction {
    pub fn new(d: Vector) -> Direction {
        Direction { d, inv: inv(&d) }
    }
}

#[inline]
pub fn len(v: &Vector) -> f64 {
    let squaresum: f64 = v.component_mul(v).iter().sum();
    squaresum.sqrt()
}

#[inline]
pub fn inv(v: &Vector) -> Vector {
    Vector::new(1.0/v.x, 1.0/v.y, 1.0/v.z)
}
