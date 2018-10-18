// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
pub use cgmath::prelude::*;
use cgmath::{ Point3, Vector3, BaseFloat };
pub mod normal;
pub mod bounds;
pub mod transform;

pub use self::transform::Trans;

pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Normal = normal::Normal3<f64>;
pub type Bounds = bounds::Bounds3<f64>;
pub type Transformation = transform::Transform3<f64>;

#[inline]
pub fn abs(v: &Vector) -> Vector {
    v.map(|c| c.abs())
}

#[inline]
pub fn lerp<N: BaseFloat>(t: N, p0: N, p1: N) -> N {
    p0 * (N::one() - t) + p1 * t
}

#[inline]
pub fn point_lerp<N: BaseFloat>(t: N, p0: &Point3<N>, p1: &Point3<N>) -> Point3<N> {
    p0 * (N::one() - t) + p1.to_vec() * t
}

#[inline]
pub fn max_dimension(v: &Vector) -> usize {
    if v.x > v.y { if v.x > v.z { 0 } else { 2 } }
    else { if v.y > v.z { 1 } else { 2 } }
}
