// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
use na;

pub type Point = na::Point3<f64>;
pub type Vector = na::Vector3<f64>;
pub type Color = na::Vector3<f64>;

#[inline]
pub fn len(v: &Vector) -> f64 {
    let squaresum: f64 = v.component_mul(v).iter().sum();
    squaresum.sqrt()
}

#[inline]
pub fn inv(v: &Vector) -> Vector {
    Vector::new(1.0/unzerofy(v.x), 1.0/unzerofy(v.y), 1.0/unzerofy(v.z))
}

// If a floating poing number is 0, makes it 1
#[inline]
fn unzerofy(n: f64) -> f64 {
    if n == 0.0 { 1.0 } else { n }
}
