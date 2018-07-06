use space::*;
use shape::{Shape, Intersection};

/**
Representation of a triangle mesh
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

/**
A Triangle consists of 3 vertex references
*/
pub struct Triangle(Point, Point, Point);

impl Mesh {
    pub fn new(positions: &[[f64; 3]], faces: &[[usize; 3]]) -> Mesh {
        Mesh {
            v: positions.iter().map(|pos| { Point::new(pos[0], pos[1], pos[2]) }).collect(),
            f: Vec::from(faces)
        }
    }

    pub fn face(&self, index: usize) -> Triangle {
        let [i1, i2, i3] = self.f[index];
        Triangle(self.v[i1], self.v[i2], self.v[i3])
    }

}

impl Shape for Mesh {
    fn intersect(&self, e: &Point, d: &Direction) -> Intersection {
        for i in 0..self.f.len() {
            let triangle = self.face(i);
            let intersection = triangle.intersect(e, d);
            if intersection.exists() { return intersection }
        }
        Intersection::none()
    }
}

impl Shape for Triangle {
    fn intersect(&self, _e: &Point, _d: &Direction) -> Intersection {
        Intersection::none()
    }
}
