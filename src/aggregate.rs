use cgmath::{Deg, Transform as CgTransform };

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
    transform: Transform
}

impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate {
            contents: vec![],
            background: Background::black(),
            transform: Transform::identity()
        }
    }

    pub fn new_with_background(color: [f64; 3]) -> Aggregate {
        let color = Color::new(color[0], color[1], color[2]);
        Aggregate {
            contents: vec![],
            background: Background::new(color),
            transform: Transform::identity()
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

    pub fn add_mesh(&mut self, vertices: Box<[f32]>, faces: Box<[u32]>, material: MaterialRef) {
        let shape = Mesh::new(vertices, faces);
        self.contents.push(Box::new(Geometry { shape, material }))
    }

    #[inline]
    pub fn translate(&mut self, delta: [f64; 3]) -> &mut Self {
        let delta = Vector::new(delta[0], delta[1], delta[2]);
        self.transform.concat_self(&Transform::translate(delta)); self
    }

    #[inline]
    pub fn scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.transform.concat_self(&Transform::scale(x, y, z)); self
    }

    #[inline]
    pub fn rotate_x(&mut self, theta: f64) -> &mut Self {
        self.transform.concat_self(&Transform::rotate_x(Deg(theta))); self
    }

    #[inline]
    pub fn rotate_y(&mut self, theta: f64) -> &mut Self {
        self.transform.concat_self(&Transform::rotate_y(Deg(theta))); self
    }

    #[inline]
    pub fn rotate_z(&mut self, theta: f64) -> &mut Self {
        self.transform.concat_self(&Transform::rotate_z(Deg(theta))); self
    }

    #[inline]
    pub fn rotate(&mut self, theta: f64, axis: Vector) -> &mut Self {
        self.transform.concat_self(&Transform::rotate(Deg(theta), axis)); self
    }
}

impl Primitive for Aggregate {
    #[inline]
    fn material(&self, _scene: &Scene) -> &dyn Material {
        &self.background
    }

    fn intersect(&self, ray: &Ray) -> (Intersection, &dyn Primitive) {
        let ray = self.transform.transform_ray_to_model(*ray);
        let init: (Intersection, &dyn Primitive) = (Intersection::none(), self);

        // Find the closest child with which this node intersects
        let (intersection, primitive) = self.contents.iter().fold(init, |closest, node| {
            let next = node.intersect(&ray);
            if next.0.t < closest.0.t { next } else { closest }
        });

        // Transform normal before sending it back
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
        assert_eq!(intersection.normal, Normal::new(Vector::new(0.0, 0.0, -1.0)))
    }
}
