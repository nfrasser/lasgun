use std::{io, path::Path, f64};

use crate::space::*;
use crate::light::{Light, point::PointLight};
use crate::material::{Material, phong::Phong};
use crate::shape::mesh::Mesh;

/// Opaque reference to a material within a scene. May be passed around and
/// copied freely but is not relevant outside the noted scene.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MaterialRef(usize);

/// Opaque reference to a .object file mesh in a scene
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjRef(usize);

/**
    Description of the world to render
*/
pub struct Scene {

    /// Additional scene rendering options
    pub options: Options,

    /// The primitives to use in the scene
    pub root: Aggregate,

    pub materials: Vec<Box<dyn Material>>, // available materials for primitives in the scene
    pub lights: Vec<Box<dyn Light>>, // point-light sources in the scene
    pub meshes: Vec<Mesh>,

    pub ambient: Color, // ambient lighting

    /// The position of the eye/camera in the scene
    pub eye: Point,

    /// The direction in which the view vector points.
    /// Its magnitude is the distance to the focal plane
    pub view: Vector,

    /// The orientation of the eye in the scene
    pub up: Vector,

    /// Auxilary vectory, orthogonal to the up and view vectors
    pub aux: Vector,

    /// Half the distance between two primrary ray intersection points on the focal plane.
    /// Primary rays with supersampling enabled will sample points around the original ray / intersection.
    /// The maximum distance of the sample ray interesection points on the focal plane from the
    /// original point of intersection will be no larger than this number.
    pub pixel_radius: f64,

    /// Precomputed supersampling options
    pub supersampling: Sampling
}

/// User-configurable description of the scene to render, passed to the scene
/// contructor.
#[derive(Clone, Copy, Debug)]
pub struct Options {
    pub eye: [f64; 3], // Point
    pub view: [f64; 3], // Vector
    pub up: [f64; 3], // Vector

    /// Ambient lighting
    /// Each element represent the RGB value and ranges from 0 to 1
    pub ambient: [f64; 3],

    // Final width/height of generated image
    pub width: u16,
    pub height: u16,

    /// Field of View angle
    pub fov: f64,

    /// Represents the number of times a pixel will be divided for supersampling
    /// operations. e.g., 0 means one sample, 1 means 4 samples, 2 means 9
    /// samples, etc.
    pub supersampling: u8,

    /// Number of CPU render threads to use. Setting this to 0 means default to
    /// the system concurrency, if available.
    pub threads: u8,
}

/// Pre-computed supersampling settings for a pixel
#[derive(Debug)]
pub struct Sampling {
    /// The number of sections the pixel is divided into in one dimension
    pub dim: u8,

    /// The total number of super samples to take
    /// Defined as the square of (dim + 1)
    pub count: usize,

    /// Half the dimension of two supersample cells.
    /// Supersampling is implemented by dividing each pixel into a grid and taking a random sample
    /// from each cell.
    pub radius: f64,

    /// A number getween 0 and 1 representing the contribution of a single supersample ray.
    pub power: f64,
}

impl Scene {
    pub fn new(options: Options) -> Scene {
        // The Auxilary Vector is normal to the view and up vectors
        let height = options.height;

        let eye = Point::new(options.eye[0], options.eye[1], options.eye[2]);
        let view = Vector::new(options.view[0], options.view[1], options.view[2]);
        let up = Vector::new(options.up[0], options.up[1], options.up[2]);

        let aux = view.cross(up);
        let up = aux.cross(view).normalize();
        let aux = aux.normalize();

        // First point of the target plane will be at from the eye
        let distance = view.magnitude();

        // Half the height of the point grid in model coordinates
        let ymax = distance * f64::tan((1.0/360.0) * options.fov * f64::consts::PI);

        // Distance between sample points on focal plane
        let pixel_radius = ymax / (height as f64);

        // The number of cells on one side of the supersample grid
        let supersample_dim = options.supersampling + 1;

        // How much to scale the sample radius by when performing supersample calculations
        let supersample_scale = 1.0/(supersample_dim as f64);

        // Total Number of samples to take
        let supersample_count = supersample_dim as usize * supersample_dim as usize;

        // How much each supersample should could for
        // computed once here so it doesn't have to be recomputed later
        let supersample_power = 1.0/(supersample_count as f64);

        Scene {
            lights: vec![], materials: vec![], meshes: vec![],
            ambient: Color::new(options.ambient[0], options.ambient[1], options.ambient[2]),
            root: description::Aggregate::new(),
            eye, view, up, aux, pixel_radius,
            supersampling: Sampling {
                dim: supersample_dim,
                count: supersample_count,
                radius: pixel_radius * supersample_scale,
                power: supersample_power
            },
            options
        }
    }

    /// Create a trivial scene for testing
    pub fn trivial() -> Scene {
        Scene::new(Options {
            eye: [0.0, 0.0, -1.0],
            view: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
            fov: 45.0,
            ambient: [0.0, 0.0, 0.0],
            width: 1,
            height: 1,
            supersampling: 0,
            threads: 0
        })
    }

    pub fn add_phong_material(&mut self, kd: [f64; 3], ks: [f64; 3], shininess: i32) -> MaterialRef {
        let material: Phong = Phong::new(kd, ks, shininess);
        self.add_material(Box::new(material))
    }

    pub fn add_point_light(&mut self, position: [f64; 3], intensity: [f64; 3], falloff: [f64; 3]) {
        let light = PointLight::new(position, intensity, falloff);
        self.lights.push(Box::new(light))
    }


    pub fn add_mesh(&mut self, mesh: Mesh) -> ObjRef {
        let reference = ObjRef(self.meshes.len());
        self.meshes.push(mesh);
        reference
    }

    /// Add a mesh from a obj file loaded as a string
    pub fn add_mesh_from(&mut self, obj: &str) -> io::Result<ObjRef> {
        let reference = ObjRef(self.meshes.len());
        self.meshes.push(Mesh::from(obj)?);
        Ok(reference)
    }

    // Add the .obj file mesh at the given file-system path
    pub fn add_mesh_at(&mut self, obj_path: &Path) -> io::Result<ObjRef> {
        let reference = ObjRef(self.meshes.len());
        self.meshes.push(Mesh::load(obj_path)?);
        Ok(reference)
    }

    pub fn set_root(&mut self, node: description::Aggregate) {
        self.root = node
    }

    pub fn material(&self, material: &MaterialRef) -> Option<&dyn Material> {
        debug_assert!(material.0 < self.materials.len());
        if let Some(material) = self.materials.get(material.0) {
            Some(&**material)
        } else {
            None
        }
    }

    pub fn mesh<'a>(&'a self, mesh: &ObjRef) -> Option<&'a Mesh> {
        self.meshes.get(mesh.0)
    }

    fn add_material(&mut self, material: Box<dyn Material>) -> MaterialRef {
        let reference = MaterialRef(self.materials.len());
        self.materials.push(material);
        reference
    }
}

pub mod description;
pub use self::description::*;
