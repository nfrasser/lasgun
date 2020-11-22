use std::{f64, path::Path};

use obj::Obj;
use crate::space::*;
use crate::camera::Camera;
use crate::light::{Light, point::PointLight};
use crate::material::Background;
use crate::shape::triangle::*;

/// Description of the world to render and how it should be rendered
pub struct Scene {

    /// Node
    pub root: Aggregate,

    /// Camera settings
    pub camera: Camera,

    /// Background material
    pub background: Background,

    /// Ambient lighting
    pub ambient: Color,

    /// Enable normal smoothing for triangle meshes that support it
    pub smoothing: bool,

    /// Maximum depth of ray recursion, defaults to 3
    pub recursion: u32,

    /// Number of parallel render threads, if applicable. Zero means use as many
    /// threads as the system allows (bin feature required)
    pub threads: usize,

    // Point-light sources in the scene (more formats to come)
    lights: Vec<Box<dyn Light>>,

    /// Available triangle mesh instances
    meshes: Vec<Obj>,
}

/// Opaque reference to a .obj-powered file mesh in a scene
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObjRef(usize);

/// User-configurable description of the scene to render, passed to the scene
/// contructor.

impl Scene {
    pub fn new() -> Scene {
        Scene {
            root: Aggregate::new(),
            camera: Camera::default(),
            background: Background::solid(Color::zero()),
            ambient: Color::new(0., 0., 0.),
            smoothing: true,
            recursion: 3,
            threads: 0,
            lights: vec![],
            meshes: vec![],
        }
    }

    pub fn set_camera(&mut self, camera: Camera) -> &mut Camera {
        self.camera = camera;
        return &mut self.camera
    }

    pub fn set_perspective_camera(&mut self, fov: f64) -> &mut Camera {
        self.camera = Camera::perspective(fov);
        return &mut self.camera
    }

    pub fn set_orthographic_camera(&mut self, scale: f64) -> &mut Camera {
        self.camera = Camera::orthographic(scale);
        return &mut self.camera
    }

    pub fn set_solid_background(&mut self, color: [f64; 3]) {
        self.background = Background::solid(color.into())
    }

    pub fn set_radial_background(&mut self, inner: [f64; 3], outer: [f64; 3], scale: f64) {
        self.background = Background::radial(inner.into(), outer.into(), scale)
    }

    pub fn set_ambient_light(&mut self, color: [f64; 3]) {
        self.ambient = color.into()
    }

    pub fn set_mesh_smoothing(&mut self, enabled: bool) {
        self.smoothing = enabled
    }

    pub fn set_max_recursion_depth(&mut self, max_depth: u32) {
        self.recursion = max_depth
    }

    pub fn set_threads(&mut self, threads: usize) {
        self.threads = threads
    }

    pub fn add_point_light(&mut self, position: [f64; 3], intensity: [f64; 3], falloff: [f64; 3]) {
        let light = PointLight::new(position, intensity, falloff);
        self.lights.push(Box::new(light))
    }

    /// Add the given loaded Obj instance to the scene
    pub fn add_obj(&mut self, mesh: Obj) -> ObjRef {
        let mut mesh = mesh;
        if !self.smoothing { mesh.data.normal.clear() };
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
}

pub mod node;
pub use self::node::*;
