// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
pub use cgmath::prelude::*;
use cgmath::{ Point3, Vector3 };
mod normal;
mod bounds;

pub type Point = Point3<f64>;
pub type Vector = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type Normal = normal::Normal3<f64>;
pub type Bounds = bounds::Bounds3<f64>;
