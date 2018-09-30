use crate::math;
use crate::space::*;
use crate::ray::Ray;
use crate::shape::{Shape, Intersection};

/**
    A sphere of any size positioned somewhere in 3D space
*/
#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: [f64; 3], radius: f64) -> Sphere{
        Sphere {
            center: Point::new(center[0], center[1], center[2]),
            radius
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let d = &ray.d;
        let rad = &self.radius;
        let cen = &self.center;

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
        if numroots == 2 {
            // Ray goes through the sphere twice
            let (t0, t1) = (roots[0].min(roots[1]), roots[0].max(roots[1]));

            // Check relative intersection distances
            if t1 < 0.0 {
                // Intersection occurs behind the ray
                Intersection::none()
            } else if t0 < 0.0 {
                // Intersects in front and behind, eye is inside the sphere!
                let normal = Normal::new(cen - (ray.origin + t1*d));
                Intersection { t: t1, normal }
            } else {
                // Eye is outside the sphere, use closest root
                let normal = Normal::new(ray.origin + t0*d - cen);
                Intersection { t: t0, normal }
            }
        } else if numroots == 1 && roots[0] > 0.0 {
            let normal = Normal::new(ray.origin + roots[0]*d - cen);
            Intersection { t: roots[0], normal }
        } else {
            Intersection::none()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn straight_on_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0);
        let origin = Point::new(0.0, 0.0, 2.0);
        let ray = Ray::new(origin, Vector::new(0.0, 0.0, -1.0));
        let intersection = sphere.intersect(&ray);

        assert!(intersection.exists());
        assert_eq!(intersection.t, 1.0);
    }
}
