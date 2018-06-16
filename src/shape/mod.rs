use std::f64;
use space::{ Point, Vector };

pub trait Shape {
    fn intersect(&self, e: &Point, d: &Vector) -> Intersection;
}

/**
    An intersection with a Ray (as defined above) is defined by t, the distance from the (E)ye, and
    the normal vector representing at the surface of intersection
*/
pub struct Intersection {
    pub t: f64, // distance to the eye
    pub normal: Vector // normal at the point of intersection
}

impl Intersection {
    pub fn none() -> Intersection {
        Intersection {
            t: f64::INFINITY,
            normal: Vector::zeros()
        }
    }

    // An Intersection exists when the distance is in the range (0, INFINITY)
    pub fn exists(&self) -> bool {
        self.t > 0.0 && self.t < f64::INFINITY
    }
}

mod sphere;
