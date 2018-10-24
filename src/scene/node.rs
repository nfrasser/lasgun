// This module contains structures for providing a simple representation of the
// contents of a scene. The elements here are later used to build up a full scene

use cgmath::{prelude::*, Deg};
use crate::{space::*};
use super::{ObjRef as Obj, MaterialRef as Material};

pub enum SceneNode {
    /// A geometric shape and a material
    Geometry(Shape, Material),
    /// Triangle mesh
    Mesh(Obj, Material),
    /// A collection of multiple scene nodes
    Group(Aggregate)
}

pub enum Shape {
    /// Sphere with origin and radius
    Sphere([f64; 3], f64),
    /// Cube with origin and dimensions from that origin
    Cube([f64; 3], f64),
    /// Similar to cube: a rectagular prism with start and end corners
    Cuboid([f64; 3], [f64; 3])
}

pub struct Aggregate {
    pub contents: Vec<SceneNode>,
    pub transform: Transformation
}

impl Aggregate {
     pub fn new() -> Aggregate {
        Aggregate {
            contents: vec![],
            transform: Transformation::identity()
        }
    }

    pub fn add(&mut self, node: SceneNode) {
        self.contents.push(node)
    }

    pub fn add_group(&mut self, aggregate: Aggregate) {
        self.add(SceneNode::Group(aggregate))
    }

    pub fn add_sphere(&mut self, center: [f64; 3], radius: f64, material: Material) {
        let shape = Shape::Sphere(center, radius);
        self.add(SceneNode::Geometry(shape, material))
    }

    pub fn add_cube(&mut self, origin: [f64; 3], dim: f64, material: Material) {
        let shape = Shape::Cube(origin, dim);
        self.add(SceneNode::Geometry(shape, material))
    }

    pub fn add_box(&mut self, minbound: [f64; 3], maxbound: [f64; 3], material: Material) {
        let shape = Shape::Cuboid(minbound, maxbound);
        self.add(SceneNode::Geometry(shape, material))
    }

    pub fn add_mesh(&mut self, mesh: Obj, material: Material) {
        self.add(SceneNode::Mesh(mesh, material))
    }

    #[inline]
    pub fn translate(&mut self, delta: [f64; 3]) -> &mut Self {
        let delta = Vector::new(delta[0], delta[1], delta[2]);
        self.transform.concat_self(&Transformation::translate(delta)); self
    }

    #[inline]
    pub fn scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.transform.concat_self(&Transformation::scale(x, y, z)); self
    }

    #[inline]
    pub fn rotate_x(&mut self, theta: f64) -> &mut Self {
        self.transform.concat_self(&Transformation::rotate_x(Deg(theta))); self
    }

    #[inline]
    pub fn rotate_y(&mut self, theta: f64) -> &mut Self {
        self.transform.concat_self(&Transformation::rotate_y(Deg(theta))); self
    }

    #[inline]
    pub fn rotate_z(&mut self, theta: f64) -> &mut Self {
        self.transform.concat_self(&Transformation::rotate_z(Deg(theta))); self
    }

    #[inline]
    pub fn rotate(&mut self, theta: f64, axis: [f64; 3]) -> &mut Self {
        let axis = Vector { x: axis[0], y: axis[1], z: axis[2] };
        self.transform.concat_self(&Transformation::rotate(Deg(theta), axis)); self
    }
}
