use std::path::Path;
use cgmath::{prelude::*, Deg};
use crate::space::*;


use crate::ray::Ray;
use crate::material::{Material, background::Background};
use crate::primitive::{Primitive, geometry::Geometry};
use crate::shape::{Intersection, mesh::Mesh, sphere::Sphere, cuboid::Cuboid};
use crate::scene::{Scene, MaterialRef};

/// A primitive that contains many primitives, all of which may be intersected
/// with. If no intersection occurs with the primitives in the content, we say
/// the intersection happens with Aggregate at t = INFINITY and the Background
/// material is used.
pub struct Aggregate {
    contents: Vec<Box<dyn Primitive>>,
    background: Background,
    transform: Transformation
}

impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate {
            contents: vec![],
            background: Background::black(),
            transform: Transformation::identity()
        }
    }

    pub fn new_with_background(color: [f64; 3]) -> Aggregate {
        let color = Color::new(color[0], color[1], color[2]);
        Aggregate {
            contents: vec![],
            background: Background::new(color),
            transform: Transformation::identity()
        }
    }

    pub fn add_aggregate(&mut self, aggregate: Aggregate) {
        self.contents.push(Box::new(aggregate))
    }

    pub fn add_sphere(&mut self, center: [f64; 3], radius: f64, material: MaterialRef) {
        let shape = Sphere::new(center, radius);
        self.contents.push(Box::new(Geometry { shape, material }))
    }

    pub fn add_cube(&mut self, origin: [f64; 3], dim: f64, material: MaterialRef) {
        let shape = Cuboid::cube(origin, dim);
        self.contents.push(Box::new(Geometry { shape, material }))
    }

    pub fn add_box(&mut self, minbound: [f64; 3], maxbound: [f64; 3], material: MaterialRef) {
        let shape = Cuboid::new(minbound, maxbound);
        self.contents.push(Box::new(Geometry { shape, material }))
    }

    pub fn add_mesh(&mut self, shape: Mesh, material: MaterialRef) {
        self.contents.push(Box::new(Geometry { shape, material }))
    }

    /// Add a mesh from a obj file loaded as a string
    pub fn add_mesh_from(&mut self, obj: &str, material: MaterialRef) {
        let shape = Mesh::from(obj).unwrap();
        self.contents.push(Box::new(Geometry { shape, material }))
    }

    // Add the .obj file mesh at the given file-system path
    pub fn add_mesh_at(&mut self, obj_path: &Path, material: MaterialRef) {
        let shape = Mesh::load(obj_path).unwrap();
        self.contents.push(Box::new(Geometry { shape, material }))
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

impl Primitive for Aggregate {
    #[inline]
    fn material(&self, _scene: &Scene) -> &dyn Material {
        &self.background
    }

    fn intersect(&self, r: &Ray) -> (Intersection, &dyn Primitive) {
        let ray = self.transform.inverse_transform_ray(*r);
        let init: (Intersection, &dyn Primitive) = (Intersection::none(), self);

        // Find the closest child with which this node intersects
        let (intersection, primitive) = self.contents.iter().fold(init, |closest, node| {
            let next = node.intersect(&ray);
            if next.0.t < closest.0.t { next } else { closest }
        });

        // Transformation normal before sending it back
        let normal = self.transform.transform_normal(intersection.normal);
        (Intersection { t: intersection.t, normal }, primitive)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn make_simple_scene() -> Scene {
        let mut scene = Scene::trivial();
        let mat = scene.add_phong_material([0.0, 0.0, 0.0], [0.0, 0.0, 0.0], 1);
        scene.contents.add_sphere([0.0, 2.0, 2.0], 1.0, mat);
        scene.contents.translate([0.0, -2.0, 0.0]);
        scene
    }

    #[test]
    fn simple_transform() {
        let scene = make_simple_scene();

        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let (intersection, _) = scene.contents.intersect(&ray);

        assert!(intersection.exists());
        assert_eq!(intersection.t, 1.0);
        assert_eq!(intersection.normal, normal::Normal3(Vector::new(0.0, 0.0, -1.0)))
    }
}
