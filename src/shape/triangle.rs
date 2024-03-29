// use std::ops::Index;
use std::{path::{Path, PathBuf}, io::{self, BufRead, BufReader}, fs::File};
use obj::Obj;

use crate::{
    space::*,
    primitive::{Primitive, OptionalPrimitive},
    interaction::RayIntersection,
    Material
};

/// A triangle references its parent mesh and the index within the faces array.
/// The triangle's lifetime depends on the mesh it references.
///
/// This implementation ensures the smallest possible triangle implementation
/// when storing large triangle meshes in memory (16 bytes on 64 bit systems)
#[derive(Copy, Clone)]
pub struct Triangle<'a> {

    /// Object index within .obj mesh
    object: u16,

    /// Group index within the object
    group: u16,

    /// Polygon index into the group. Contains face data as TriangleIndex
    poly: u32,

    /// Reference to the parsed .obj file mesh that contains this triangle
    /// Used to extract information like vertex positions
    obj: &'a Obj,
}

impl<'a> Triangle<'a> {
    pub fn new(obj: &'a Obj, object: u16, group: u16, poly: u32) -> Triangle<'a> {
        Triangle { object, group, poly, obj }
    }

    #[inline]
    pub fn p0(&self) -> Point {
        let v = self.obj.data.position[((self.poly().0)[0]).0];
        Point::new(v[0].into(), v[1].into(), v[2].into())
    }

    #[inline]
    pub fn p1(&self) -> Point {
        let v = self.obj.data.position[((self.poly().0)[1]).0];
        Point::new(v[0].into(), v[1].into(), v[2].into())
    }

    #[inline]
    pub fn p2(&self) -> Point {
        let v = self.obj.data.position[((self.poly().0)[2]).0];
        Point::new(v[0].into(), v[1].into(), v[2].into())
    }

    #[inline]
    pub fn n0(&self) -> Vector {
        debug_assert!(self.has_n());
        let n = self.obj.data.normal[(((self.poly().0)[0]).2).unwrap()];
        Vector::new(n[0].into(), n[1].into(), n[2].into())
    }

    #[inline]
    pub fn n1(&self) -> Vector {
        debug_assert!(self.has_n());
        let n = self.obj.data.normal[(((self.poly().0)[1]).2).unwrap()];
        Vector::new(n[0].into(), n[1].into(), n[2].into())
    }

    #[inline]
    pub fn n2(&self) -> Vector {
        debug_assert!(self.has_n());
        let n = self.obj.data.normal[(((self.poly().0)[2]).2).unwrap()];
        Vector::new(n[0].into(), n[1].into(), n[2].into())
    }

    /// Find all triangle UV texture coordinates
    #[inline]
    pub fn uv(&self) -> [Point2f; 3] {
        if self.has_uv() {
            [self.uv0(), self.uv1(), self.uv2()]
        } else {
            [
                Point2f { x: 0.0, y: 0.0 },
                Point2f { x: 1.0, y: 0.0 },
                Point2f { x: 1.0, y: 1.0 }
            ]
        }
    }

    #[inline]
    pub fn uv0(&self) -> Point2f {
        debug_assert!(self.has_uv());
        let tuple = (self.poly().0)[0];
        let uv = self.obj.data.texture[(tuple.1).unwrap()];
        Point2f::new(uv[0].into(), uv[1].into())
    }

    #[inline]
    pub fn uv1(&self) -> Point2f {
        debug_assert!(self.has_uv());
        let tuple = (self.poly().0)[1];
        let uv = self.obj.data.texture[(tuple.1).unwrap()];
        Point2f::new(uv[0].into(), uv[1].into())
    }

    #[inline]
    pub fn uv2(&self) -> Point2f {
        debug_assert!(self.has_uv());
        let tuple = (self.poly().0)[2];
        let uv = self.obj.data.texture[(tuple.1).unwrap()];
        Point2f::new(uv[0].into(), uv[1].into())
    }

    // Whether this mesh has normals mapped
    #[inline]
    pub fn has_n(&self) -> bool {
        self.obj.data.normal.len() > 0
    }

    // Whether this mesh has UV texture coordinates mapped
    #[inline]
    pub fn has_uv(&self) -> bool {
        self.obj.data.texture.len() > 0
    }

    #[inline]
    fn object(&self) -> &obj::Object {
        &self.obj.data.objects[self.object as usize]
    }

    #[inline]
    fn group(&self) -> &obj::Group {
        &self.object().groups[self.group as usize]
    }

    #[inline]
    fn poly(&self) -> &obj::SimplePolygon {
        &self.group().polys[self.poly as usize]
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

impl<'a> Primitive for Triangle<'a> {
    fn bound(&self) -> Bounds {
        Bounds::new(self.p0(), self.p1()).point_union(&self.p2())
    }

    fn intersect(&self, ray: &Ray, isect: &mut RayIntersection) -> OptionalPrimitive {
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
            return None
        };

        // If the sum is 0, the triangle is right on the edge
        let det = e0 + e1 + e2;
        if det == 0.0 { return None };

        // Compute scaled hit distance to triangle and test against ray t range
        // This step was saved from later
        p0t.z *= sz;
        p1t.z *= sz;
        p2t.z *= sz;
        let tscaled = e0 * p0t.z + e1 * p1t.z + e2 * p2t.z;

        // Check for mismatched determinant and tscaled signs (triangle is behind ray)
        if (det < 0.0 && tscaled >= 0.0) || (det > 0.0 && tscaled <= 0.0) {
            return None
        }

        // compute barycentric coordinates and t value for triangle intersection
        // barycentric coordinates can be used to "interpolate" the sheared z value across the triangle
        let invdet = 1.0 / det;
        let b0 = e0 * invdet;
        let b1 = e1 * invdet;
        let b2 = e2 * invdet;
        let t = tscaled * invdet;
        if t >= isect.t { return None };

        // TODO: ensure that computed triangle t is conservatively greater than 0

        // TODO: shading normals

        // 3. Compute triangle partial derivatives
        let uv = self.uv();
        let duv02 = uv[0] - uv[2]; let duv12 = uv[1] - uv[2];
        let dp02 = p0 - p2; let dp12 = p1 - p2;
        let determinant = (duv02.x * duv12.y) - (duv02.y * duv12.x);
        let (dpdu, dpdv) = if determinant == 0.0 {
            coordinate_system(&(p2 - p1).cross(p1 - p0))
        } else {
            let invdet = 1.0 / determinant;
            (
                (duv12.y * dp02 - duv02.y * dp12) * invdet,
                (-duv12.x * dp02 - duv02.x * dp12) * invdet
            )
        };

        // Hit uv point

        // TODO: 4. compute error bounds for triangle intersections
        // 5. Interpolate (u, v) parametric coordinates (hit point determined later)
        let uv = b0 * uv[0].add_element_wise(b1 * uv[1]).add_element_wise(b2 * uv[2]);

        // TODO: 6. Test intersection against alpha texture, if present

        // 7. fill in Intersection from triangle hit
        // There is for sure an intersection at this point, compute the normal from original points
        *isect = RayIntersection::new(t, uv, dpdu, dpdv);

        if self.has_n() {
            // Compute shading normal ns, surface tangent ss for triangle
            let (n0, n1, n2) = (self.n0(), self.n1(), self.n2());
            let ns = b0 * n0 + b1 * n1 + b2 * n2;
            let ss = isect.geometry.dpdu;

            // Compute shading tangent ss for triangle
            let ts = ns.cross(ss);
            let (ss, ts) = if ts.magnitude2() > 0.0 {
                (ts.cross(ns), ts)
            } else {
                coordinate_system(&ns)
            };

            isect.n = Some(normal::Normal3(ns));
            isect.set_surface_shading(ss, ts)
        } else {
            // Set to default normal so that it faces towards the ray.
            // This prevents invalid shading on meshes with missing normals
            isect.n = Some(normal::Normal3(dp02.cross(dp12)).face_forward(-ray.d));
        }

        Some(self)
    }

    // TODO: Grab a material from the loaded Mtl libraries if one is available
    fn material(&self) -> Option<Material> { None }
}

/// Structure that allows using a obj as an iterator
/// Each item in the iterator is a triangle that references the parent obj
pub struct TriangleIterator<'a> {
    obj: &'a Obj,
    // Current iteration indeces
    size_hint: usize,
    object_index: usize,
    group_index: usize,
    poly_index: usize
}

impl<'a> TriangleIterator<'a> {
    pub fn new(obj: &'a Obj) -> TriangleIterator<'a> {
        TriangleIterator {
            obj,
            size_hint: face_count(obj),
            object_index: 0,
            group_index: 0,
            poly_index: 0,
        }
    }
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = Triangle<'a>;

    fn next(&mut self) -> Option<Triangle<'a>> {
        if self.size_hint == 0 { return None };
        let triangle = Triangle::new(
            &self.obj,
            self.object_index as u16,
            self.group_index as u16,
            self.poly_index as u32);

        self.poly_index += 1;

        if self.poly_index == self.obj.data.objects[self.object_index].groups[self.group_index].polys.len() {
            self.poly_index = 0;
            self.group_index += 1;
        }

        if self.group_index == self.obj.data.objects[self.object_index].groups.len() {
            self.group_index = 0;
            self.object_index += 1;
        }

        if self.object_index == self.obj.data.objects.len() {
            self.size_hint = 0;
        } else {
            self.size_hint -= 1;
        }

        return Some(triangle)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size_hint, Some(self.size_hint))
    }
}

/// Load from an object file at the given path
#[inline]
pub fn load_obj(path: &Path) -> Result<Obj, obj::ObjError> {
    let f = File::open(path)?;
    let mut obj = obj_from_buf(&mut BufReader::new(f))?;
    // unwrap is safe as we've read this file before
    obj.path = path.parent().unwrap().to_owned();
    Ok(obj)
}

/// Parse the string contents of a .obj file into a `Obj` instance.
#[inline]
pub fn parse_obj(slice: &str) -> Result<Obj, obj::ObjError> {
    let mut buf = io::Cursor::new(slice);
    obj_from_buf(&mut buf)
}

/// Parse the given readable buffer of a .obj file into a `Obj` instance.
#[inline]
pub fn obj_from_buf<B>(input: &mut B) -> Result<Obj, obj::ObjError> where B: BufRead {
    let data = obj::ObjData::load_buf(input)?;
    Ok(Obj { data, path: PathBuf::new() })
}

/// Number of faces on this obj
pub fn face_count(obj: &Obj) -> usize {
    obj.data.objects.iter().fold(0, |size, object| {
        object.groups.iter().fold(size, |size, group| {
            size + group.polys.len()
        })
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plane_intersection() {
        let plane = parse_obj(r#"o plane
v -1 0 -1
v 1 0 -1
v 1 0 1
v -1 0 1

f 1 2 3
f 1 3 4
"#
        ).unwrap();

        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let mut isect = RayIntersection::default();
        for triangle in TriangleIterator::new(&plane) {
            triangle.intersect(&ray, &mut isect);
        }

        assert_eq!(isect.t, 1.0);
        assert_eq!(isect.ng(), Vector::unit_y());
    }

    #[test]
    fn plane_intersection_with_normals_and_texture() {
        let plane = parse_obj(r#"o plane
v -1 0 -1
v 1 0 -1
v 1 0 1
v -1 0 1

f 1 2 3
f 1 3 4
"#
        ).unwrap();

        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let mut isect = RayIntersection::default();
        for triangle in TriangleIterator::new(&plane) {
            triangle.intersect(&ray, &mut isect);
        }

        assert_eq!(isect.t, 1.0);
        assert_eq!(isect.ng(), Vector::unit_y());
    }
}
