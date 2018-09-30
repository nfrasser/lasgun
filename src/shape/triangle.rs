// use std::ops::Index;

use crate::space::*;
use crate::ray::Ray;

use super::{Shape, Intersection};
use super::mesh::Mesh;

/// A triangle references its parent mesh and the index within the faces array. The triangle's
/// lifetime depends on the mesh it references. This implementation ensures the smallest possible
/// triangle implementation when storing large triangle meshes in memory.
#[derive(Debug)]
pub struct Triangle<'a> {
    /**
    Reference to the mesh that contains this triangle
    */
    mesh: &'a Mesh,

    /**
    Index within `f` list of vertex indeces in the mesh vertex data
    */
    f: u32
}

impl<'a> Triangle<'a> {
    pub fn new(mesh: &'a Mesh, f: u32) -> Triangle<'a> {
        debug_assert!(f < mesh.fcount());
        Triangle { mesh, f }
    }

    /// Get the point at the given index
    #[inline]
    fn p(&self, i: usize) -> Point {
        debug_assert!(i < 3);
        let vertices = self.mesh.vertices();
        let faces = self.mesh.faces();
        let vertex = faces[3 * self.f as usize + i] as usize;

        let (vx, vy, vz) = (
            vertices[vertex * 3 + 0],
            vertices[vertex * 3 + 1],
            vertices[vertex * 3 + 2],
        );

        Point::new(vx.into(), vy.into(), vz.into())
    }

    #[inline]
    fn p0(&self) -> Point {
        self.p(0)
    }

    #[inline]
    fn p1(&self) -> Point {
        self.p(1)
    }

    #[inline]
    fn p2(&self) -> Point {
        self.p(2)
    }
}

// TODO: Make it possible to return a &Point somehow
/*
impl<'a> Index<usize> for Triangle<'a> {
    type Output = Point;
    #[inline]
    fn index(&self, index: usize) -> &Point {
        debug_assert!(index < 3);
        &self.p(index)
    }
}
*/

impl<'a> Shape for Triangle<'a> {
    fn intersect(&self, ray: &Ray) -> Intersection {
        // 1. Get triangle vertices
        let (p0, p1, p2) = (self.p0(), self.p1(), self.p2());

        // 2. Perform ray-triangle intersection
        // Transform triangle vertices to ray coordinate space
        // This reduces the problem to determining whether the origin (0, 0) lies within
        // the triangle made up of the (x, y) components of the transformed 3D triangle

        // There are 3 steps to this:
        // 1. Translate to ray origin (T)
        // 2. Permute coordinates (P) - ensures intersection succeeds if ray's z comp is 0
        // 3. Shear with ray direction (S)
        // Rather than computing M = SPT, we use this more effictient approach

        // basically translate triangle by -(ray origin) and rotate so that ray looks down along z+ axis ===

        // Translate vertices based on ray origin
        let (p0t, p1t, p2t) = (
            Point::from_vec(p0 - ray.origin),
            Point::from_vec(p1 - ray.origin),
            Point::from_vec(p2 - ray.origin),
        );

        // Permute components of triangle vertices and ray direction
        let kz = max_dimension(&abs(&ray.d)); // component with max absolute value (0 to 2)
        let kx = (kz + 1) % 3; // choose x/y arbitrarly based on x
        let ky = (kx + 1) % 3;
        let d: Vector = permute!(Vector, ray.d, kx, ky, kz);

        let (mut p0t, mut p1t, mut p2t) = (
            permute!(Point, p0t, kx, ky, kz),
            permute!(Point, p1t, kx, ky, kz),
            permute!(Point, p2t, kx, ky, kz),
        );

        // Apply shear transformation to translated vertex position
        // TODO: Pre-compute these in the ray struct for all permutations
        let sx = -d.x / d.z;
        let sy = -d.y / d.z;
        let sz = 1.0 / d.z;

        // Only x, y sheared for now
        // we'll do z after if an intersection actually occurs
        p0t.x += sx * p0t.z;
        p0t.y += sy * p0t.z;
        p1t.x += sx * p1t.z;
        p1t.y += sy * p1t.z;
        p2t.x += sx * p2t.z;
        p2t.y += sy * p2t.z;

        // compute edge function coefficiens e0 e1 e2
        // Whether these are <0 or >0 represent the side of the edge the origin is on
        // These are derived from the function for the area of a triangle defined by 3 vertexes,
        // where one of them is the origin
        let (e0, e1, e2) = (
            p1t.x * p2t.y - p1t.y * p2t.x,
            p2t.x * p0t.y - p2t.y * p0t.x,
            p0t.x * p1t.y - p0t.y * p1t.x,
        );

        // Perform triangle edge and determinant tests
        if (e0 < 0.0 || e1 < 0.0 || e2 < 0.0) && (e0 > 0.0 || e1 > 0.0 || e2 > 0.0) {
            // Edge coefficients differ, origin is outside
            return Intersection::none()
        };

        // If the sum is 0, the triangle is right on the edge
        let det = e0 + e1 + e2;
        if det == 0.0 { return Intersection::none() };

        // Compute scaled hit distance to triangle and test against ray t range
        // This step was saved from later
        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;
        let tscaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;

        // Check for mismatched determinant and tscaled signs (triangle is behind ray)
        if (det < 0.0 && tscaled >= 0.0) || (det > 0.0 && tscaled <= 0.0) {
            return Intersection::none()
        }

        // compute barycentric coordinates and t value for triangle intersection
        // barycentric coordinates can be used to "interpolate" the sheared z value across the triangle
        let invdet = 1.0 / det;
        // let b0 = e0 * invdet;
        // let b1 = e1 * invdet;
        // let b2 = e2 * invdet;
        let t = tscaled * invdet;

        // TODO: ensure that computed triangle t is conservatively greater than 0

        // TODO: 3. Compute triangle partial derivatives
        // TODO: 4. compute error bounds for triangle intersections
        // TODO: 5. Interpolate (u, v) parametric coordinates and hit point
        // TODO: 6. Test intersection against alpha texture, if present

        // 7. fill in Intersection from triangle hit
        // There is for sure an intersection at this point, compute the normal from original points
        let normal = (p2 - p1).cross(p1 - p0);
        if normal.dot(ray.d) > 0.0 {
            Intersection::new(t, -normal)
        } else {
            Intersection::new(t, normal)
        }
    }
}
