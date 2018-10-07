use crate::space::*;
use crate::shape::*;
use super::triangle::Triangle;

/**
Container representing a triangle mesh. A reference to it
is stored by all triangles available in the scene

You can iterate over all of a mesh's triangles by calling the into_iter method
*/
#[derive(Debug)]
pub struct Mesh {

    /// The vertices that make up this mesh.
    /// Each group of 3 represents an vertex position.
    vertices: Box<[f32]>,

    /// Each element is an index * 3 into the vertices array.
    /// Each group of 3 represents indeces to 3 verteces that make up a face.
    faces: Box<[u32]>,

    /// Mesh bounding box
    pub bounds: Bounds
}

impl Mesh {
    pub fn new(vertices: Box<[f32]>, faces: Box<[u32]>) -> Mesh {
        // Must have 3 floats per vertex
        // Must also have 3 indeces into the vertex array per face
        assert!(vertices.len() % 3 == 0);
        assert!(faces.len() % 3 == 0);

        // Generate the bounds
        let bounds = vertices.chunks(3)
        .fold(Bounds::none(), |bounds, vertex| {
            let point = Point::new(vertex[0].into(), vertex[1].into(), vertex[2].into());
            bounds.point_union(&point)
        });

        Mesh { vertices, faces, bounds }
    }

    #[inline]
    pub fn vertices(&self) -> &[f32] {
        &*self.vertices
    }

    #[inline]
    pub fn faces(&self) -> &[u32] {
        &*self.faces
    }

    /// The total number of vertices in this mesh
    #[inline]
    pub fn vcount(&self) -> u32 {
        (self.vertices.len() / 3) as u32
    }

    /// The total number of faces in this mesh
    #[inline]
    pub fn fcount(&self) -> u32 {
        (self.faces.len() / 3) as u32
    }
}

impl Shape for Mesh {
    fn intersect(&self, ray: &Ray) -> Intersection {
        // Check if intersects with bounding box before doing triangle intersection
        if !self.bounds.intersect(ray).exists() { return Intersection::none() }

        let init = Intersection::none();
        self.into_iter().fold(init, |closest, triangle| {
            let next = triangle.intersect(ray);
            if next.t < closest.t { next } else { closest }
        })
    }
}

impl<'a> IntoIterator for &'a Mesh {
    type Item = Triangle<'a>;
    type IntoIter = MeshIterator<'a>;

    fn into_iter(self) -> Self::IntoIter { MeshIterator::new(self) }
}

/// Structure that allows using a mesh as an iterator
/// Each item in the iterator is a triangle that references the parent mesh
pub struct MeshIterator<'a> {
    pub mesh: &'a Mesh,
    fcount: u32, // total number of faces
    findex: u32 // current face index
}

impl<'a> MeshIterator<'a> {
    fn new(mesh: &'a Mesh) -> MeshIterator<'a> {
        MeshIterator { mesh, fcount: mesh.fcount(), findex: 0 }
    }
}

impl<'a> Iterator for MeshIterator<'a> {
    type Item = Triangle<'a>;

    fn next(&mut self) -> Option<Triangle<'a>> {
        if self.findex < self.fcount {
            let f = self.findex;
            self.findex += 1;
            Some(Triangle::new(self.mesh, f))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.fcount - self.findex;
        (remaining as usize, Some(remaining as usize))
    }
}

mod test {
    use super::*;

    #[test]
    fn plane_intersection() {
        let plane = Mesh::new(
            Box::new([
                -1.0, 0.0, -1.0,
                1.0, 0.0, -1.0,
                1.0, 0.0, 1.0,
                -1.0, 0.0, 1.0,
            ]),
            Box::new([
                0, 2, 1,
                0, 3, 2,
            ]));

        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let intersection = plane.intersect(&ray);

        assert_eq!(intersection.t, 1.0);
        assert_eq!(intersection.normal.0.normalize(), Vector::unit_y());
    }
}
