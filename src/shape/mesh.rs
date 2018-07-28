use space::*;
use shape::*;
use super::triangle::Triangle;

/**
Container representing a triangle mesh. A reference to it
is stored by all triangles available in the scene

You can iterate over all of a mesh's triangles by calling the into_iter method
*/
pub struct Mesh {

    /**
    The vertices that make up this mesh
    */
    pub vertices: Vec<Point>,

    /**
    The triangular faces formed by the vertices.
    Each element represents 3 indeces within the vertices vector
    */
    pub faces: Vec<[usize; 3]>
}

impl Mesh {
    pub fn new(positions: &[[f64; 3]], faces: &[[usize; 3]]) -> Mesh {
        Mesh {
            vertices: positions.iter()
                .map(|p| Point::new(p[0], p[1], p[2]))
                .collect(),
            faces: Vec::from(faces)
        }
    }
}

impl Shape for Mesh {
    fn intersect(&self, ray: &Ray) -> Intersection {
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
    fcount: usize, // total number of faces
    findex: usize // current face index
}

impl<'a> MeshIterator<'a> {
    fn new(mesh: &'a Mesh) -> MeshIterator<'a> {
        MeshIterator { mesh, fcount: mesh.faces.len(), findex: 0 }
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
        (remaining, Some(remaining))
    }
}
