use space::*;
use ray::Ray;

use std::ops::Index;
use std::rc::Rc;
use shape::{Shape, Intersection};


/**
A triangle references its parent mesh and the index within
the faces array
*/
pub struct Triangle {
    /**
    Reference to the mesh this triangle contains
    */
    mesh: Rc<Mesh>,

    /**
    Reference to list of vertex indeces in the mesh vertex data
    */
    f: usize
}

impl Triangle {
    pub fn new(mesh: Rc<Mesh>, f: usize) -> Triangle {
        Triangle { mesh, f }
    }
}

impl Shape for Triangle {
    fn intersect(&self, ray: &Ray) -> Intersection {
    }
}

/**
Container representing a triangle mesh. A reference to it
is stored by all triangles available in the scene
*/
pub struct Mesh {

    /**
    The verteces that make up this mesh
    */
    pub v: Vec<Point>,

    /**
    The triangular faces formed by the vertices.
    Each element represents 3 indeces within the v vector
    */
    pub f: Vec<[usize; 3]>
}

impl Mesh {
    pub fn new(positions: &[[f64; 3]], faces: &[[usize; 3]]) -> Mesh {
        Mesh {
            v: positions.iter()
                .map(|p| Point::new(p[0], p[1], p[2]))
                .collect(),
            f: Vec::from(faces)
        }
    }
}
