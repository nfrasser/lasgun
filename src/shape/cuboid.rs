use std::f64;

use crate::space::*;
use crate::ray::Ray;
use crate::interaction::SurfaceInteraction;
use super::{Primitive, Shape};

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

impl Primitive for Cuboid {
    fn bound(&self) -> Bounds {
        self.bounds.bound()
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        self.bounds.intersect(ray, interaction)
    }
    fn intersects(&self, ray: &Ray) -> bool {
        self.bounds.intersects(ray)
    }
}

impl Shape for Cuboid {}

impl Primitive for Bounds {
    fn bound(&self) -> Bounds { *self }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let mut tnear = f64::NEG_INFINITY;
        let mut tfar = f64::INFINITY;

        let mut normal = Vector::zero();

        // i ranges from X to Z
        for i in 0..3 {
            let t1 = (self.min[i] - ray.origin[i]) * ray.dinv[i];
            let t2 = (self.max[i] - ray.origin[i]) * ray.dinv[i];

            let tmin = t1.min(t2);
            let tmax = t1.max(t2);

            if tmin > tnear {
                normal = Vector::new(CUBE_NORMALS[i][0], CUBE_NORMALS[i][1], CUBE_NORMALS[i][2]);
            }

            tnear = tnear.max(tmin);
            tfar = tfar.min(tmax);
        }

        // Check if out of bounds
        if tnear > tfar || tfar <= 0.0 { return false }

        // Intersection, check if it happens behind the ray and set t accordingly
        let t = if tnear < 0.0 { tfar } else { tnear };
        if t >= interaction.t { return false }

        interaction.t = t;
        // interaction.p = ray.origin + ray.d * t;
        interaction.n = normal::Normal3(normal).face_forward(ray.d);

        true
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let mut tnear = f64::NEG_INFINITY;
        let mut tfar = f64::INFINITY;

        // i ranges from X to Z
        for i in 0..3 {
            let t1 = (self.min[i] - ray.origin[i]) * ray.dinv[i];
            let t2 = (self.max[i] - ray.origin[i]) * ray.dinv[i];

            let tmin = t1.min(t2);
            let tmax = t1.max(t2);

            tnear = tnear.max(tmin);
            tfar = tfar.min(tmax);
        }

        tnear <= tfar && tfar > 0.0
    }
}

impl Shape for Bounds {}

// Vectors representing the cube normals
const CUBE_NORMALS: [[f64; 3]; 3] = [
    [1.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
    [0.0, 0.0, 1.0]
];

#[cfg(test)]
mod test {
    use super::*;
    // use cgmath::prelude::*;

    #[test]
    fn straight_on_intersection() {
        let cube = Cuboid::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]);
        let ray = Ray::new(Point::new(0.0, 0.0, -2.0), Vector::new(0.0, 0.0, 1.0), 0);
        let mut interaction = SurfaceInteraction::default();

        assert!(cube.intersect(&ray, &mut interaction));
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n, Normal::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn edge_intersection() {
        let cube = Cuboid::new([-1.1, -1.1, -1.0], [1.1, 1.1, 1.0]);
        let ray = Ray::new(Point::new(0.0, 0.0, -2.0), Vector::new(1.0, 0.0, 1.0), 0);
        let mut interaction = SurfaceInteraction::default();

        assert!(cube.intersect(&ray, &mut interaction));
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n, Normal::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn corner_intersection() {
        let cube = Cuboid::new([-1.1, -1.1, -1.0], [1.1, 1.1, 1.0]);
        let ray = Ray::new(Point::new(0.0, 0.0, -2.0), Vector::new(1.0, 1.0, 1.0), 0);
        let mut interaction = SurfaceInteraction::default();

        assert!(cube.intersect(&ray, &mut interaction));
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n, Normal::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn inside_intersection() {
        let cube = Cuboid::new([-1.0, -1.0, -1.0], [1.0, 1.0, 1.0]);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::unit_z(), 0);
        let mut interaction = SurfaceInteraction::default();

        assert!(cube.intersect(&ray, &mut interaction));
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n, Normal::new(0.0, 0.0, -1.0));
    }
}
