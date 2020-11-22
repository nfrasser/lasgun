use std::f64::{NEG_INFINITY, consts::PI};
use crate::core::math;
use crate::space::*;
use crate::primitive::{Primitive, OptionalPrimitive};
use crate::interaction::RayIntersection;
use crate::Material;

/**
    A sphere of any size positioned somewhere in 3D space
*/
#[derive(Debug)]
pub struct Sphere {
    pub origin: Point,
    pub radius: f64,
    pub material: Material
}

impl Sphere {
    pub fn new(origin: [f64; 3], radius: f64, material: Material) -> Sphere {
        Sphere {
            origin: Point::new(origin[0], origin[1], origin[2]),
            radius,
            material
        }
    }

    /// Returns the parametric t at the point of intersection. Negative values
    /// mean no intersection. The second return parameter is true if the
    /// intersection happens inside the sphere
    fn intersect_t(&self, ray: &Ray) -> (f64, bool) {
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
            if t0 < 0.0 { (t1, true) } else { (t0, false) }
        } else if numroots == 1 {
            (roots[0], false)
        } else {
            (NEG_INFINITY, false)
        }
    }
}

impl Primitive for Sphere {
    fn bound(&self) -> Bounds {
        Bounds::new(
            self.origin - Vector::from_value(self.radius),
            self.origin + Vector::from_value(self.radius))
    }

    fn intersect(&self, ray: &Ray, isect: &mut RayIntersection) -> OptionalPrimitive {
        let (t, inside) = self.intersect_t(ray);

        // Intersection behind the ray, do nothing
        if t < 0.0 { return None; }

        // A better intersection was already found, continue
        if t >= isect.t { return None }

        // The ray definitely intersects with the sphere, calcuate shading
        // parameters.

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
        let dpdu = Vector {
            x: -2.0 * PI * p.y,
            y: 2.0 * PI * p.x,
            z: 0.0
        };

        let dpdv = PI * Vector {
            x: p.z * phi.cos(),
            y: p.z * phi.sin(),
            z: -self.radius * theta.sin()
        };

        // Swap if outside the sphere
        let (dpdu, dpdv) = if inside { (dpdu, dpdv) } else { (dpdv, dpdu) };

        // FIXME: Get correct UVs
        *isect = RayIntersection::new(t, Point2f::new(0.0, 0.0), dpdu, dpdv);

        Some(self)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        self.intersect_t(ray).0 >= 0.0
    }

    fn material(&self) -> Option<Material> { Some(self.material) }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn straight_on_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0, Material::default());
        let origin = Point::new(0.0, 0.0, 2.0);
        let ray = Ray::new(origin, Vector::new(0.0, 0.0, -1.0));
        let mut isect = RayIntersection::default();

        assert!(sphere.intersect(&ray, &mut isect).is_some());
        assert_eq!(isect.t, 1.0);
        assert_eq!(isect.ng(), Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn inside_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0, Material::default());
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let mut isect = RayIntersection::default();

        assert!(sphere.intersect(&ray, &mut isect).is_some());
        assert_eq!(isect.t, 1.0);
        assert_eq!(isect.ng(), Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn behind_intersection() {
        let sphere = Sphere::new([0.0, 0.0, 0.0], 1.0, Material::default());
        let origin = Point::new(0.0, 0.0, -2.0);
        let ray = Ray::new(origin, Vector::new(0.0, 0.0, 1.0));
        let mut isect = RayIntersection::default();

        assert!(sphere.intersect(&ray, &mut isect).is_some());
        assert_eq!(isect.t, 1.0);

        // Round otherwise ends up too close to zero
        let ng = isect.ng();
        let ng = Vector::new(ng.x.round(), ng.y.round(), ng.z.round());
        assert_eq!(ng, Vector::new(0.0, 0.0, -1.0));
    }
}
