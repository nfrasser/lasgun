use space::*;
use ray::Ray;

use material::{ Material, background::Background };
use shape::Intersection;

use super::Primitive;


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

    pub fn add(&mut self, primitive: Box<Primitive>) where {
        self.contents.push(primitive)
    }
}

impl Primitive for Aggregate {
    fn material(&self) -> &Material {
        &self.background
    }

    fn intersect(&self, ray: &Ray) -> (Intersection, &Primitive) {
        let init: (Intersection, &Primitive) = (Intersection::none(), self);

        // Find the closest child with which this node intersects
        self.contents.iter().fold(init, |closest, node| {
            let next = node.intersect(ray);
            if next.0.t < closest.0.t { next } else { closest }
        })
    }
}
