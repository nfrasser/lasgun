use std::{path::Path, io::{self, BufRead, BufReader}, fs::File};

use crate::{space::*, interaction::SurfaceInteraction};

use super::*;
use super::triangle::*;

/// A triangle mesh loaded from a .obj file
pub struct Mesh {
    pub obj: Obj
}


/// Container representing a triangle mesh.
/// You can iterate over all of a mesh's triangles by calling the into_iter method

impl Mesh {
    pub fn new(obj: Obj) -> Mesh {
        Mesh { obj }
    }

    /// Load from an object file at the given path
    pub fn load(path: &Path) -> io::Result<Mesh> {
        let f = File::open(path)?;
        let mut obj = Obj::load_buf(&mut BufReader::new(f))?;
        // unwrap is safe as we've read this file before
        obj.path = path.parent().unwrap().to_owned();
        Ok(Mesh::new(obj))
    }

    pub fn load_buf<B>(input: &mut B) -> io::Result<Self> where B: BufRead {
        let obj = Obj::load_buf(input)?;
        Ok(Mesh::new(obj))
    }

    pub fn from(slice: &str) -> io::Result<Self> {
        let mut buf = io::Cursor::new(slice);
        Mesh::load_buf(&mut buf)
    }

    /// Number of faces on this mesh
    pub fn fcount(&self) -> usize {
        self.obj.objects.iter().fold(0, |size, object| {
            object.groups.iter().fold(size, |size, group| {
                size + group.polys.len()
            })
        })
    }
}

// NOTE: This implementation is not used. BVH hierarchy construction is
// responsible for this.
impl Primitive for Mesh {
    fn object_bound(&self) -> Bounds {
        self.obj.position.iter().fold(Bounds::none(), |bounds, pos| {
            bounds.point_union(&Point::new(pos[0].into(), pos[1].into(), pos[2].into()))
        })
    }

    fn intersect(&self, ray: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        self.into_iter().fold(false, |exists, triangle| {
            triangle.intersect(ray, interaction) || exists
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
    obj: &'a Obj,
    // Current iteration indeces
    size_hint: usize,
    object_index: usize,
    group_index: usize,
    poly_index: usize
}

impl<'a> MeshIterator<'a> {
    fn new(mesh: &'a Mesh) -> MeshIterator<'a> {
        MeshIterator {
            obj: &mesh.obj,
            size_hint: mesh.fcount(),
            object_index: 0,
            group_index: 0,
            poly_index: 0,
        }
    }
}

impl<'a> Iterator for MeshIterator<'a> {
    type Item = Triangle<'a>;

    fn next(&mut self) -> Option<Triangle<'a>> {
        if self.size_hint == 0 { return None };
        let triangle = Triangle::new(
            &self.obj,
            self.object_index as u16,
            self.group_index as u16,
            self.poly_index as u32);

        self.poly_index += 1;

        if self.poly_index == self.obj.objects[self.object_index].groups[self.group_index].polys.len() {
            self.poly_index = 0;
            self.group_index += 1;
        }

        if self.group_index == self.obj.objects[self.object_index].groups.len() {
            self.group_index = 0;
            self.object_index += 1;
        }

        if self.object_index == self.obj.objects.len() {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plane_intersection() {
        let plane = Mesh::from(r#"o plane
v -1 0 -1
v 1 0 -1
v 1 0 1
v -1 0 1

f 1 3 2
f 1 4 3
"#
        ).unwrap();

        let ray = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        let mut interaction = SurfaceInteraction::none();

        assert!(plane.intersect(&ray, &mut interaction));
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n.0.normalize(), Vector::unit_y());
    }
}
