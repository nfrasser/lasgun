use shape::mesh::Mesh;

use ray::Ray;
use scene::Scene;
use shape::{
    Shape, Intersection,
    sphere::Sphere,
    cuboid::Cuboid
};
use material::Material;
use super::Primitive;

/// A primitive representing a geometric shape such as a sphere or cube.
/// The intersection is computed mathematically.
///
/// A geometry holds the index of a material in a given scene.
pub struct Geometry {
    pub shape: Box<Shape>,
    material: usize
}

impl Geometry {
    pub fn sphere(center: [f64; 3], radius: f64, material: usize) -> Geometry {
        let sphere = Sphere::new(center, radius);
        Geometry { shape: Box::new(sphere), material }
    }

    pub fn cube(origin: [f64; 3], dim: f64, material: usize) -> Geometry {
        let cube = Cuboid::cube(origin, dim);
        Geometry { shape: Box::new(cube), material }
    }

    /// Triangle mesh
    pub fn mesh(mesh: Mesh, material: usize) -> Geometry {
        Geometry { shape: Box::new(mesh), material }
    }
}

impl Primitive for Geometry {
    #[inline]
    fn material<'a>(&self, scene: &'a Scene) -> &'a Material {
        debug_assert!(self.material < scene.options.materials.len());
        // Unchecked get to avoid strict bounds checking and improve performance
        unsafe { &**scene.options.materials.get_unchecked(self.material) }
    }

    fn intersect(&self, ray: &Ray) -> (Intersection, &Primitive) {
        (self.shape.intersect(ray), self)
    }
}
