use std::rc::Rc;
use primitive::Primitive;
use space::*;

use primitive::geometry::Geometry;
use material::Material;
use material::background::Background;
use shape::{
    Intersection,
    triangle::{
        Triangle,
        Mesh
    }
};


/**
    A primitive that contains many primitives, all of which may be intersected with. If no
    intersection occurs with the primitives in the content, we say the intersection happens with
    Aggregate at t = INFINITY and the Background material is used.
*/
pub struct Aggregate {
    contents: Vec<Box<Primitive>>,
    background: Background,

    // Transformation matrix
    // transform: Matrix4<f64>,

    // Inverse transformation matrix
    // invert: Matrix4<f64>
}

impl Aggregate {
    pub fn new(contents: Vec<Box<Primitive>>, background: Color) -> Aggregate {
        Aggregate { contents, background: Background::new(background) }
    }

    pub fn just(contents: Vec<Box<Primitive>>) -> Aggregate {
        Aggregate::new(contents, Color::zeros())
    }

    /**
    Create an aggregate of Triangle shapes from the given mesh
    */
    pub fn triangles(mesh: Mesh, material: Rc<Material>) -> Aggregate {
        let nfaces = mesh.f.len();
        let mesh = Rc::new(mesh);
        let contents = (0..nfaces)
            .map(|i| Triangle::new(mesh.clone(), i))
            .map(|t| Geometry::triangle(t, material.clone()))
            .map(|g| Box::new(g) as Box<Primitive>)
            .collect();

        Aggregate::just(contents)
    }

    pub fn add(&mut self, primitive: Box<Primitive>) where {
        self.contents.push(primitive)
    }
}

impl Primitive for Aggregate {
    fn material(&self) -> &Material {
        &self.background
    }

    fn intersect(&self, e: &Point, d: &Direction) -> (Intersection, &Primitive) {
        let init: (Intersection, &Primitive) = (Intersection::none(), self);

        // Find the closest child with which this node intersects
        self.contents.iter().fold(init, |closest, node| {
            let next = node.intersect(e, d);
            if next.0.t < closest.0.t { next } else { closest }
        })
    }
}
