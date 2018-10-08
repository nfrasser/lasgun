use std::f64;
use crate::space::*;
use crate::ray::Ray;

/**
    A gemetric shape that lives in 3D space.
    Must implement a way to intersect with a ray defined by the given eye and direction vector
*/
pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Intersection;
}

/**
    An intersection with a Ray (as defined above) is defined by
    - t: the distance from the (e)ye as a ratio of the length of the direction vector
    - normal: vector at the surface of intersection
*/
pub struct Intersection {
    pub t: f64, // distance to the eye based on direction vector
    pub normal: Normal // normal at the point of intersection
}

impl Intersection {
    pub fn new(t: f64, normal: Normal) -> Intersection {
        Intersection { t, normal }
    }

    pub fn none() -> Intersection {
        Intersection {
            t: f64::INFINITY,
            normal: normal::Normal3(Vector::zero())
        }
    }

    // An Intersection exists when the distance is in the range (0, INFINITY)
    #[inline]
    pub fn exists(&self) -> bool {
        self.t > 0.0 && self.t < f64::INFINITY
    }
}

pub mod cuboid;
pub mod sphere;
pub mod mesh;
pub mod triangle;
