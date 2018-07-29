// Contains shortcuts for commonly used linear-algebra types used in the ray-tracer
use na;
mod normal;
mod bounds;

pub type Point = na::Point3<f64>;
pub type Vector = na::Vector3<f64>;
pub type Color = na::Vector3<f64>;
pub type Normal = normal::Normal3<f64>;
pub type Bounds = bounds::Bounds3<f64>;
