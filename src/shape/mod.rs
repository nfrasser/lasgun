use crate::ray::Ray;
pub use crate::primitive::Primitive;

/// A gemetric shape that lives in 3D space. Must implement a way to intersect
/// with a given ray
pub trait Shape: Primitive {}

pub mod cuboid;
pub mod sphere;
pub mod mesh;
pub mod triangle;

pub use self::cuboid::Cuboid;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;
