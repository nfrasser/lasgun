use std::sync::Arc;
use shape::mesh::Mesh;

use ray::Ray;
use shape::{
    Shape, Intersection,
    sphere::Sphere,
    cuboid::Cuboid
};
use material::Material;
use super::Primitive;

/**
    A primitive representing a geometric shape such as a sphere or cube.
    The intersection is computed mathematically
*/
pub struct Geometry {
    pub shape: Box<Shape>,
    pub material: Arc<Material>
}

impl Geometry {
    pub fn sphere(center: [f64; 3], radius: f64, material: Arc<Material>) -> Geometry {
        let sphere = Sphere::new(center, radius);
        Geometry { shape: Box::new(sphere), material }
    }

    pub fn cube(origin: [f64; 3], dim: f64, material: Arc<Material>) -> Geometry {
        let cube = Cuboid::cube(origin, dim);
        Geometry { shape: Box::new(cube), material }
    }

    /// Triangle mesh
    pub fn mesh(mesh: Mesh, material: Arc<Material>) -> Geometry {
        Geometry { shape: Box::new(mesh), material }
    }
}

impl Primitive for Geometry {
    fn material(&self) -> &Material {
        &*self.material
    }

    fn intersect(&self, ray: &Ray) -> (Intersection, &Primitive) {
        (self.shape.intersect(ray), self)
    }
}

unsafe impl Sync for Geometry {}
