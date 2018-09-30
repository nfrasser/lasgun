use crate::space::*;
use crate::ray::Ray;

use crate::material::{Material, background::Background};
use crate::primitive::{Primitive, geometry::Geometry};
use crate::shape::{Shape, Intersection, mesh::Mesh, sphere::Sphere, cuboid::Cuboid};
use crate::scene::{Scene, MaterialRef};

/// A primitive that contains many primitives, all of which may be intersected
/// with. If no intersection occurs with the primitives in the content, we say
/// the intersection happens with Aggregate at t = INFINITY and the Background
/// material is used.
pub struct Aggregate {
    contents: Vec<Box<dyn Primitive>>,
    background: Background,

    // Transformation matrix
    // transform: Matrix4<f64>,

    // Inverse transformation matrix
    // invert: Matrix4<f64>
}

impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate { contents: vec![], background: Background::black() }
    }

    pub fn new_with_background(color: [f64; 3]) -> Aggregate {
        let color = Color::new(color[0], color[1], color[2]);
        Aggregate {
            contents: vec![],
            background: Background::new(color)
        }
    }

    pub fn add_sphere(&mut self, center: [f64; 3], radius: f64, material: MaterialRef) {
        let sphere = Sphere::new(center, radius);
        self.add_shape(Box::new(sphere), material);
    }

    pub fn add_cube(&mut self, origin: [f64; 3], dim: f64, material: MaterialRef) {
        let cube = Cuboid::cube(origin, dim);
        self.add_shape(Box::new(cube), material);
    }

    pub fn add_box(&mut self, minbound: [f64; 3], maxbound: [f64; 3], material: MaterialRef) {
        let cube = Cuboid::new(minbound, maxbound);
        self.add_shape(Box::new(cube), material);
    }

    pub fn add_mesh(&mut self, vertices: Box<[f32]>, faces: Box<[u32]>, material: MaterialRef) {
        let mesh = Mesh::new(vertices, faces);
        self.add_shape(Box::new(mesh), material);
    }

    fn add_shape(&mut self, shape: Box<dyn Shape>, material: MaterialRef) {
        let primitive = Geometry { shape, material };
        self.contents.push(Box::new(primitive))
    }
}

impl Primitive for Aggregate {
    #[inline]
    fn material(&self, _scene: &Scene) -> &dyn Material {
        &self.background
    }

    fn intersect(&self, ray: &Ray) -> (Intersection, &dyn Primitive) {
        let init: (Intersection, &dyn Primitive) = (Intersection::none(), self);

        // Find the closest child with which this node intersects
        self.contents.iter().fold(init, |closest, node| {
            let next = node.intersect(ray);
            if next.0.t < closest.0.t { next } else { closest }
        })
    }
}
