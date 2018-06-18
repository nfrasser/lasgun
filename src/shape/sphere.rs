use space::*;
use math;
use shape::Shape;
use shape::Intersection;

/**
    A sphere of any size positioned somewhere in 3D space
*/
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
    fn intersect(&self, e: &Point, direction: &Direction) -> Intersection {
        let d = &direction.d;
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
        let l: Vector = e - cen;

        // A, B, and C expand to the following:
        let a = d.dot(d);
        let b = 2.0 * d.dot(&l);
        let c = l.dot(&l) - rad*rad;

        // Calculate the roots
        let (roots, numroots) = math::quad_roots(a, b, c);

        // Find the closest point of intersection, it available
        if numroots == 2 {
            // Ray goes through the sphere twice
            let (t0, t1) = (
                roots[0].min(roots[1]),
                roots[0].max(roots[1])
            );

            // Check relative intersection distances
            if t1 < 0.0 {
                // Intersection occurs behind the ray
                Intersection::none()
            } else if t0 < 0.0 {
                // Intersects in front and behind, eye is inside the sphere!
                Intersection { t: t1, normal: cen - (e + t1*d) }
            } else {
                // Eye is outside the sphere, use closest root
                Intersection { t: t0, normal: e + t0*d - cen }
            }
        } else if numroots == 1 && roots[0] > 0.0 {
            Intersection { t: roots[0], normal: e + roots[0]*d - cen }
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
        let e = Point::new(0.0, 0.0, 2.0);
        let d = Direction::new(Vector::new(0.0, 0.0, -1.0));
        let intersection = sphere.intersect(&e, &d);
        assert!(intersection.exists());
        assert_eq!(intersection.t, 1.0);
    }
}
