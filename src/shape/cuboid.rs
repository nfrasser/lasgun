use std::f64;

use space::*;
use ray::Ray;
use shape::Shape;
use shape::Intersection;

/**
aka "Box", aka "Rectangular prism"
*/
pub struct Cuboid {
    pub bounds: (Point, Point)
}

impl Cuboid {
    pub fn new(minbound: [f64; 3], maxbound: [f64; 3]) -> Cuboid {
        let minbound = Point::new(minbound[0], minbound[1], minbound[2]);
        let maxbound = Point::new(maxbound[0], maxbound[1], maxbound[2]);
        Cuboid { bounds: (minbound, maxbound) }
    }

    pub fn cube(origin: [f64; 3], dim: f64) -> Cuboid {
        let origin = Point::new(origin[0], origin[1], origin[2]);
        Cuboid { bounds: (origin, origin + Vector::repeat(dim)) }
    }
}

impl Shape for Cuboid {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let mut tnear = f64::NEG_INFINITY;
        let mut tfar = f64::INFINITY;

        let mut normal = Vector::zeros();

        // i ranges from X to Z
        for i in 0..3 {
            let t1 = (self.bounds.0[i] - ray.origin[i]) * ray.dinv[i];
            let t2 = (self.bounds.1[i] - ray.origin[i]) * ray.dinv[i];

            let tmin = t1.min(t2);
            let tmax = t1.max(t2);

            if tmin > tnear && tmin > 0.0 {
                // Intersects with the front plane, make normal to it
                normal = Vector::new(CUBE_NORMALS[i][0], CUBE_NORMALS[i][1], CUBE_NORMALS[i][2]);
            }

            tnear = tnear.max(tmin);
            tfar = tfar.min(tmax);
        }

        if tnear >= tfar {
            // No intersection
            Intersection::none()
        } else {
            // Intersection, check if it happens behind the ray
            let t = if tnear < 0.0 { tfar } else { tnear };
            Intersection::new(t, normal)
        }
    }
}

// Vectors representing the cube normals
const CUBE_NORMALS: [[f64; 3]; 3] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 1.0, 1.0]
];
