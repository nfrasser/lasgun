use ::space::{ Point, Vector };
use shape::Shape;
use shape::Intersection;

pub struct Sphere {
    pub position: Point,
    pub radius: f64
}

impl Shape for Sphere {
    fn intersect(&self, E: &Point, d: &Vector) -> Intersection {
        // TODO
        Intersection {
            t: 0.0,
            normal: Vector::zeros()
        }
    }
}
