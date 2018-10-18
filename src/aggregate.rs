use std::path::Path;
use cgmath::{prelude::*, Deg};
use crate::space::*;

use crate::ray::Ray;
use crate::primitive::{Primitive, geometry::Geometry};
use crate::shape::{mesh::Mesh, sphere::Sphere, cuboid::Cuboid};
use crate::scene::{MaterialRef};
use crate::interaction::SurfaceInteraction;

/// A primitive that contains many primitives, all of which may be intersected
/// with. If no intersection occurs with the primitives in the content, we say
/// the intersection happens with Aggregate at t = INFINITY and the Background
/// material is used.
pub struct Aggregate {
    contents: Vec<Box<dyn Primitive>>,
    transform: Transformation
}

impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate {
            contents: vec![],
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
    fn object_bound(&self) -> Bounds {
        self.contents.iter()
        .fold(Bounds::none(), |bounds, prim| bounds.union(&prim.object_bound()))
    }

    fn intersect(&self, r: &Ray, interaction: &mut SurfaceInteraction) -> bool {
        let ray = self.transform.inverse_transform_ray(*r);
        let mut i = self.transform.inverse_transform_surface_interaction(interaction);
        let mut exists = false;

        // Find the closest child with which this node intersects
        for node in self.contents.iter() {
            exists = node.intersect(&ray, &mut i) || exists
        }

        // Transform normal before sending it back
        if exists {
            interaction.t = i.t;
            interaction.n = self.transform.transform_normal(i.n);
            interaction.p = self.transform.transform_point(i.p);
            interaction.material = i.material;
        }

        exists
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scene::Scene;

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
        let mut interaction = SurfaceInteraction::none();

        assert!(scene.contents.intersect(&ray, &mut interaction););
        assert_eq!(interaction.t, 1.0);
        assert_eq!(interaction.n, normal::Normal3(Vector::new(0.0, 0.0, -1.0)))
    }
}
