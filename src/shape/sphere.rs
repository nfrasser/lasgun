use std::f64;
use crate::math;
use crate::space::*;
use crate::ray::Ray;
use crate::shape::{Primitive, Shape};
use crate::interaction::SurfaceInteraction;

/**
    A sphere of any size positioned somewhere in 3D space
*/
#[derive(Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(origin: [f64; 3], radius: f64) -> Sphere {
        Sphere {
            origin: Point::new(origin[0], origin[1], origin[2]),
            radius
        }
    }
}

impl Primitive for Sphere {
    fn bound(&self) -> Bounds {
        Bounds::new(
            self.origin - Vector::from_value(self.radius),
            self.origin + Vector::from_value(self.radius))
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let d = &ray.d;
        let rad = &self.radius;
        let cen = &self.origin;

        // Based on the equation of a sphere:
        // (x - x0)^2 + (y - y0)^2 + (z - z0)^2 = R^2

        // Let vector d = P - E
        // Let vector v = E + td

        // Sub x = v[x], y = v[y], z = v[z]

        // Then rearrange in terms of t to get
        // At^2 + Bt + C = 0

        // Vector from the eye to the centre of the sphere
        let l: Vector = ray.origin - cen;

        // A, B, and C expand to the following:
        let a = d.dot(*d);
        let b = 2.0 * d.dot(l);
        let c = l.dot(l) - rad*rad;

        // Calculate the roots
        let (roots, numroots) = math::quad_roots(a, b, c);

        // Find the closest point of intersection, it available
        let (t, normal) = if numroots == 2 {
            // Ray goes through the sphere twice
            let (t0, t1) = (roots[0].min(roots[1]), roots[0].max(roots[1]));

            // Check relative intersection distances
            if t1 < 0.0 {
                // Intersection occurs behind the ray
                (-1.0, None)
            } else if t0 < 0.0 {
                // Intersects in front and behind, eye is inside the sphere!
                let normal = normal::Normal3(cen - (ray.origin + t1*d));
                (t1, Some(normal))
            } else {
                // Eye is outside the sphere, use closest root
                let normal = normal::Normal3(ray.origin + t0*d - cen);
                (t0, Some(normal))
            }
        } else if numroots == 1 && roots[0] > 0.0 {
            let normal = normal::Normal3(ray.origin + roots[0]*d - cen);
            (roots[0], Some(normal.face_forward(ray.d)))
        } else {
            (-1.0, None)
        };

        if let Some(normal) = normal {
            if t >= interaction.t { return false }
            // A nearby interaction exists
            interaction.t = t;
            interaction.n = normal;
            true
        } else {
            false
        }
    }
}

impl Shape for Sphere {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn straight_on_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0);
        let origin = Point::new(0.0, 0.0, 2.0);
        let ray = Ray::new(origin, Vector::new(0.0, 0.0, -1.0));
        let mut interation = SurfaceInteraction::default();

        assert!(sphere.intersect(&ray, &mut interation));
        assert_eq!(interation.t, 1.0);
        assert_eq!(interation.n, Normal::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inside_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut interaction = SurfaceInteraction::default();

        assert!(sphere.intersect(&ray, &mut interaction));
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n, Normal::new(0.0, 0.0, -1.0));
    }
}
