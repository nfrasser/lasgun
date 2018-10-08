use std::f64;

use crate::space::*;
use crate::ray::Ray;
use super::{Shape, Intersection};

/**
aka "Box", aka "Rectangular prism"
*/
#[derive(Debug)]
pub struct Cuboid {
    pub bounds: Bounds
}

impl Cuboid {
    pub fn new(minbound: [f64; 3], maxbound: [f64; 3]) -> Cuboid {
        let minbound = Point::new(minbound[0], minbound[1], minbound[2]);
        let maxbound = Point::new(maxbound[0], maxbound[1], maxbound[2]);
        Cuboid { bounds: Bounds::new(minbound, maxbound) }
    }

    pub fn cube(origin: [f64; 3], dim: f64) -> Cuboid {
        let origin = Point::new(origin[0], origin[1], origin[2]);
        Cuboid { bounds: Bounds::new(origin, origin + Vector::from_value(dim)) }
    }
}

impl Shape for Cuboid {
    fn intersect(&self, ray: &Ray) -> Intersection {
        self.bounds.intersect(ray)
    }
}

impl Shape for Bounds {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mut tnear = f64::NEG_INFINITY;
        let mut tfar = f64::INFINITY;

        let mut normal = Vector::zero();

        // i ranges from X to Z
        for i in 0..3 {
            let t1 = (self.min[i] - ray.origin[i]) * ray.dinv[i];
            let t2 = (self.max[i] - ray.origin[i]) * ray.dinv[i];

            let tmin = t1.min(t2);
            let tmax = t1.max(t2);

            if tmin > tnear && tmin > 0.0 {
                // Intersects with the front plane, make normal to it
                normal = Vector::new(CUBE_NORMALS[i][0], CUBE_NORMALS[i][1], CUBE_NORMALS[i][2]);
            }

            tnear = tnear.max(tmin);
            tfar = tfar.min(tmax);
        }

        if tnear > tfar {
            // No intersection
            Intersection::none()
        } else {
            // Intersection, check if it happens behind the ray
            let t = if tnear < 0.0 { tfar } else { tnear };
            Intersection { t, normal: normal::Normal3(normal).face_forward(ray.d) }
        }
    }
}

// Vectors representing the cube normals
const CUBE_NORMALS: [[f64; 3]; 3] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0]
];
