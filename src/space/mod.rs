// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
pub use cgmath::prelude::*;
use cgmath::{ Point3, Vector3 };
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

/*
#[inline]
pub fn min_component(v: &Vector) -> f64 {
    v.x.min(v.y).min(v.z)
}

#[inline]
pub fn max_component(v: &Vector) -> f64 {
    v.x.max(v.y).max(v.z)
}
*/

#[inline]
pub fn max_dimension(v: &Vector) -> usize {
    if v.x > v.y { if v.x > v.z { 0 } else { 2 } }
    else { if v.y > v.z { 1 } else { 2 } }
}
