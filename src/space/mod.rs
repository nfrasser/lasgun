// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
pub use cgmath::prelude::*;
use cgmath::{ Point2, Point3, Vector2, Vector3, BaseFloat };
pub mod normal;
pub mod bounds;
pub mod transform;
pub mod ray;

pub use self::transform::Trans;
pub use self::ray::Ray;

pub type Point2f = Point2<f64>;
pub type Vector2f = Vector2<f64>;
pub type Vector2u = Vector2<u32>;
pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;

#[allow(dead_code)] pub type Normal = normal::Normal3<f64>; // used in tests
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
pub fn max_dimension(v: &Vector) -> usize {
    if v.x > v.y { if v.x > v.z { 0 } else { 2 } }
    else { if v.y > v.z { 1 } else { 2 } }
}

#[inline]
pub fn coordinate_system(v1: &Vector) -> (Vector, Vector) {
    let v2 = if v1.x.abs() > v1.y.abs() {
        Vector::new(-v1.z, 0.0, v1.x) / (v1.x * v1.x + v1.z * v1.z).sqrt()
    } else {
        Vector::new(0.0, v1.z, -v1.y) / (v1.y * v1.y + v1.z * v1.z).sqrt()
    };
    let v3 = v1.cross(v2);
    (v2, v3)
}
