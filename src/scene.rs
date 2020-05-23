use std::{f64, path::Path};

use obj::Obj;
use crate::space::*;
use crate::light::{Light, point::PointLight};
use crate::material::Background;
use crate::shape::triangle::*;

/// Description of the world to render and how it should be rendered
pub struct Scene {

    /// Node
    pub root: Aggregate,

    /// Additional scene rendering options
    pub options: Options,

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

    /// Half the distance between two primary ray intersection points on the
    /// focal plane. Primary rays with supersampling enabled will sample points
    /// around the original ray / intersection. The maximum distance of the
    /// sample ray interesection points on the focal plane from the original
    /// point of intersection will be no larger than this number.
    pub pixel_radius: f64,

    /// Precomputed supersampling options
    pub supersampling: Sampling,

    // Background material
    pub background: Background,

    lights: Vec<Box<dyn Light>>, // point-light sources in the scene
    meshes: Vec<Obj>,
}

/// Opaque reference to a .obj-powered file mesh in a scene
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjRef(usize);

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

    /// Enable normal smoothing for triangle meshes that support it
    pub smoothing: bool,

    /// Represents the number of times a pixel will be divided for supersampling
    /// operations. e.g., 0 means one sample, 1 means 4 samples, 2 means 9
    /// samples, etc.
    pub supersampling: u8,

    /// Number of CPU render threads to use. Setting this to 0 means default to
    /// the system concurrency, if available.
    pub threads: u8

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

        // How much each supersample should count for
        // computed once here so it doesn't have to be recomputed later
        let supersample_power = 1.0/(supersample_count as f64);

        Scene {
            root: Aggregate::new(),
            options,
            ambient: Color::new(options.ambient[0], options.ambient[1], options.ambient[2]),
            eye, view, up, aux, pixel_radius,
            supersampling: Sampling {
                dim: supersample_dim,
                count: supersample_count,
                radius: pixel_radius * supersample_scale,
                power: supersample_power
            },
            background: Background::solid(Color::zero()),
            lights: vec![], meshes: vec![],
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
            threads: 0,
            smoothing: false
        })
    }

    pub fn add_point_light(&mut self, position: [f64; 3], intensity: [f64; 3], falloff: [f64; 3]) {
        let light = PointLight::new(position, intensity, falloff);
        self.lights.push(Box::new(light))
    }

    /// Add the given loaded Obj instance to the scene
    pub fn add_obj(&mut self, mesh: Obj) -> ObjRef {
        let mut mesh = mesh;
        if !self.options.smoothing { mesh.data.normal.clear() };
        let reference = ObjRef(self.meshes.len());
        self.meshes.push(mesh);
        reference
    }

    /// Generate triangle mesh from the given string contents of a .obj file and
    /// add it to the scene. If parsed correctly, returns a reference to the
    /// mesh for use in scene node construction.
    pub fn parse_obj(&mut self, obj: &str) -> Result<ObjRef, obj::ObjError> {
        let obj = parse_obj(obj)?;
        Ok(self.add_obj(obj))
    }

    // Load the .obj file mesh at the given file-system path and add it to the
    // scene.
    pub fn load_obj(&mut self, obj_path: &Path) -> Result<ObjRef, obj::ObjError> {
        let obj = load_obj(obj_path)?;
        Ok(self.add_obj(obj))
    }

    pub fn set_root(&mut self, node: Aggregate) {
        self.root = node
    }

    pub fn lights(&self) -> &Vec<Box<dyn Light>> { &self.lights }

    /// Return a reference to the object instance for the given ObjRef, if
    /// available.
    pub fn obj<'a>(&'a self, obj: ObjRef) -> Option<&'a Obj> {
        self.meshes.get(obj.0)
    }

    pub fn set_solid_background(&mut self, color: [f64; 3]) {
        let color = Color::new(color[0], color[1], color[2]);
        self.background = Background::solid(color)
    }

    pub fn set_radial_background(&mut self, inner: [f64; 3], outer: [f64; 3]) {
        let inner = Color::new(inner[0], inner[1], inner[2]);
        let outer = Color::new(outer[0], outer[1], outer[2]);
        self.background = Background::radial(inner, outer, self.view, self.options.fov)
    }
}

pub mod node;
pub use self::node::*;
