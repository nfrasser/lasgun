use std::f64::{NEG_INFINITY, consts::PI};
use crate::core::math;
use crate::space::*;
use crate::ray::Ray;
use crate::primitive::{Primitive, OptionalPrimitive};
use crate::shape::Shape;
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

    /// Returns the parametric t at the point of intersection. Negative values
    /// mean no intersection.
    fn intersect_t(&self, ray: &Ray) -> f64 {
        let d = ray.d;
        let rad = self.radius;
        let cen = self.origin;

        // Based on the equation of a sphere:
        // (x - x0)^2 + (y - y0)^2 + (z - z0)^2 = R^2

        // Let vector d = P - E
        // Let vector v = E + td

        // Sub x = v[x], y = v[y], z = v[z]

        // Then rearrange in terms of t to get
        // At^2 + Bt + C = 0

        // Vector from the eye to the centre of the sphere
        let l = ray.origin - cen;

        // A, B, and C expand to the following:
        let a = d.dot(d);
        let b = 2.0 * d.dot(l);
        let c = l.dot(l) - rad*rad;

        // Calculate the roots
        let (roots, numroots) = math::quad_roots(a, b, c);

        // Find the closest point of intersection, it available
        if numroots == 2 {
            // Ray goes through the sphere twice
            let (t0, t1) = (roots[0].min(roots[1]), roots[0].max(roots[1]));

            // Check if ray origin is inside the sphere
            if t0 < 0.0 { t1 } else { t0 }
        } else if numroots == 1 {
            roots[0]
        } else {
            NEG_INFINITY
        }
    }
}

impl Primitive for Sphere {
    fn bound(&self) -> Bounds {
        Bounds::new(
            self.origin - Vector::from_value(self.radius),
            self.origin + Vector::from_value(self.radius))
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> OptionalPrimitive {
        let t = self.intersect_t(ray);

        // Intersection behind the ray, do nothing
        if t < 0.0 { return None; }

        // A better intersection was already found, continue
        if t >= interaction.t { return None }

        // The ray definitely intersects with the sphere, calcuate shading
        // parameters.
        interaction.t = t;

        // Subtract the origin to find intersection from the centre
        let mut p = ray.origin + ray.d * t - self.origin;

        // Account for intersection right at the top
        if p.x == 0.0 && p.y == 0.0 { p.x = 1e-5 * self.radius }

        // A sphere can be parametrized by 2D parameters (ϕ, θ) with
        // ϕ ∈ [0, 2π] and θ ∈ [0, π]
        let mut phi = p.y.atan2(p.x);
        if phi < 0.0 { phi += 2.0 * PI };
        let theta = (p.z / self.radius).max(-1.0).min(1.0).acos();

        // Derived by taking the partial derivatives of u and v
        interaction.dpdu = Vector {
            x: -2.0 * PI * p.y,
            y: 2.0 * PI * p.x,
            z: 0.0
        };

        interaction.dpdv = PI * Vector {
            x: p.z * phi.cos(),
            y: p.z * phi.sin(),
            z: -self.radius * theta.sin()
        };

        Some(self)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect_t(ray) >= 0.0
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
        let mut interaction = SurfaceInteraction::default();

        assert!(sphere.intersect(&ray, &mut interaction).is_some());
        interaction.commit(&ray);
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n(), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inside_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut interaction = SurfaceInteraction::default();

        assert!(sphere.intersect(&ray, &mut interaction).is_some());
        interaction.commit(&ray);
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n(), Vector::new(0.0, 0.0, -1.0));
    }
}
