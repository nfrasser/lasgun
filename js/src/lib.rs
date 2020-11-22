mod utils;

use cfg_if::cfg_if;
use std::mem;
use std::ops::{Index, IndexMut};
use lasgun::{self, Pixel};
use wasm_bindgen::prelude::*;
use self::utils::Native;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern {
    /// Duck-type for Scene settings representation in JavaScript
    pub type SceneSettings;
    #[wasm_bindgen(method, getter, structural)]
    pub fn ambient(this: &SceneSettings) -> Option<Box<[JsValue]>>; // Optional vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn smoothing(this: &SceneSettings) -> Option<bool>;

    /// Duck type for Camera settings in JavaScript
    pub type CameraSettings;
    #[wasm_bindgen(method, getter, structural)]
    pub fn projection(this: &CameraSettings) -> Option<String>; // "perspective" or "orthographic" (defaults to perspective)
    #[wasm_bindgen(method, getter, structural)]
    pub fn fov(this: &CameraSettings) -> Option<f64>;  // for persective only, defaults to 45
    #[wasm_bindgen(method, getter, structural)]
    pub fn scale(this: &CameraSettings) -> Option<f64>;  // for orthographic only, defaults to fov x view magic
    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &CameraSettings) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn look(this: &CameraSettings) -> Box<[JsValue]>; // Point
    #[wasm_bindgen(method, getter, structural)]
    pub fn up(this: &CameraSettings) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn supersampling(this: &CameraSettings) -> Option<u8>;
    #[wasm_bindgen(method, getter, structural)]
    pub fn aperture(this: &CameraSettings) -> Option<f64>; // Radius

    /// Duck-type Plastic material settings
    /// For JavaScript objects that have the form
    /// {
    ///     kd: [number, number, number],
    ///     ks: [number, number, number],
    ///     roughness?: number // f64, with range [0, 1], defaults to 0
    /// }
    pub type Plastic;
    #[wasm_bindgen(method, getter, structural)]
    pub fn kd(this: &Plastic) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn ks(this: &Plastic) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn roughness(this: &Plastic) -> Option<f64>;

    /// Duck-type Matte material settings
    /// For JavaScript objects that have the form
    /// {
    ///     kd: [number, number, number],
    ///     sigma?: number // f64 with range [0, 1]
    /// }
    pub type Matte;
    #[wasm_bindgen(method, getter, structural)]
    pub fn kd(this: &Matte) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn sigma(this: &Matte) -> Option<f64>;

    /// Duck-type Metal material settings
    /// For JavaScript objects that have the form
    /// {
    ///     eta: [number, number, number],
    ///     k: [number, number, number],
    ///     roughness?: number // (f64, with range [0, 1])
    ///     u_roughness?: number // (f64, with range [0, 1])
    ///     v_roughness?: number // (f64, with range [0, 1])
    /// }
    pub type Metal;
    #[wasm_bindgen(method, getter, structural)]
    pub fn eta(this: &Metal) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn k(this: &Metal) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn roughness(this: &Metal) -> Option<f64>;
    #[wasm_bindgen(method, getter, structural)]
    pub fn u_roughness(this: &Metal) -> Option<f64>;
    #[wasm_bindgen(method, getter, structural)]
    pub fn v_roughness(this: &Metal) -> Option<f64>;

    /// Duck-type Mirror material settings
    /// For JavaScript objects that have the form
    /// {
    ///     kr?: [number, number, number], (defaults to [1, 1, 1])
    /// }
    pub type Mirror;
    #[wasm_bindgen(method, getter, structural)]
    pub fn kr(this: &Mirror) -> Option<Box<[JsValue]>>; // Vector

    /// Duck-type Glass material settings
    /// For JavaScript objects that have the form
    /// {
    ///     kr?: [number, number, number], // defaults to [1, 1, 1]
    ///     kt?: [number, number, number], // defaults to [1, 1, 1]
    ///     eta?: number // f64, with range [0, 1], defaults to 1.5
    /// }
    pub type Glass;
    #[wasm_bindgen(method, getter, structural)]
    pub fn kr(this: &Glass) -> Option<Box<[JsValue]>>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn kt(this: &Glass) -> Option<Box<[JsValue]>>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn eta(this: &Glass) -> Option<f64>;

    /// Duck-type Point Light settings
    /// For JavaScript objects that have the form
    /// {
    ///     position: [number, number, number],
    ///     intensity: [number, number, number],
    ///     falloff: [number, number, number]
    /// }
    pub type PointLight;
    #[wasm_bindgen(method, getter, structural)]
    pub fn position(this: &PointLight) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn intensity(this: &PointLight) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn falloff(this: &PointLight) -> Box<[JsValue]>; // Vector

    /// Ducktype for Sphere settings
    pub type Sphere;
    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &Sphere) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn radius(this: &Sphere) -> f64;

    /// Ducktype for Cube settings
    pub type Cube;
    #[wasm_bindgen(method, getter, structural)]
    pub fn origin(this: &Cube) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn dim(this: &Cube) -> f64;

    /// Ducktype for Box settings
    pub type Cuboid;
    #[wasm_bindgen(method, getter, structural)]
    pub fn start(this: &Cuboid) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn end(this: &Cuboid) -> Box<[JsValue]>; // Vector

    // Duck type for radial background definition
    pub type RadialBackground;
    #[wasm_bindgen(method, getter, structural)]
    pub fn inner(this: &RadialBackground) -> Box<[JsValue]>; // Color
    #[wasm_bindgen(method, getter, structural)]
    pub fn outer(this: &RadialBackground) -> Box<[JsValue]>; // Color
    #[wasm_bindgen(method, getter, structural)]
    pub fn scale(this: &RadialBackground) -> Option<f64>; // Color

    fn alert(s: &str);
}

/// Alias for the scene constructor
#[wasm_bindgen]
pub fn scene(settings: &SceneSettings) -> Scene {
    Scene::new(settings)
}

/// Alias for the Camera constructor
#[wasm_bindgen]
pub fn camera(settings: &CameraSettings) -> Camera {
    Camera::new(settings)
}

/// Generate a new film for the given scene
/// Alias for the film constructor
#[wasm_bindgen]
pub fn film(width: u32, height: u32) -> Film {
    Film::new(width, height)
}

/// Capture the whole scene onto the given film
#[wasm_bindgen]
pub fn capture(scene: &Scene, film: &mut Film) {
    let accel = lasgun::Accel::from(scene.as_native());
    lasgun::capture_subset(0, 1, &accel, film)
}

/// Capture subset k ∈ [0, n-1] of n of the accelerated scene-structure onto the
/// given film
#[wasm_bindgen]
pub fn capture_subset(k: usize, n: usize, accel: &Accel, film: &mut Film) {
    lasgun::capture_subset(k, n, accel.as_native(), film)
}

// Triangle mesh reference in a scene
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct ObjRef(lasgun::scene::ObjRef); impl Native for ObjRef {
    type Output = lasgun::scene::ObjRef;
    #[inline] fn into_native(self) -> Self::Output { self.0 }
    #[inline] fn as_native(&self) -> &Self::Output { &self.0 }
    #[inline] fn as_native_mut(&mut self) -> &mut Self::Output { &mut self.0 }
}

#[wasm_bindgen]
pub struct Camera(lasgun::Camera); impl Native for Camera {
    type Output = lasgun::Camera;
    #[inline] fn into_native(self) -> Self::Output { self.0 }
    #[inline] fn as_native(&self) -> &Self::Output { &self.0 }
    #[inline] fn as_native_mut(&mut self) -> &mut Self::Output { &mut self.0 }
}

#[wasm_bindgen]
impl Camera {
    pub fn new(settings: &CameraSettings) -> Camera {
        let projection = settings.projection().unwrap_or("perspective".to_string());
        let fov = settings.fov().unwrap_or(45.);
        let scale = settings.scale().unwrap_or(1.); // TODO: Default scale from fov
        let mut camera = match projection.as_str() {
            "perspective" => lasgun::Camera::perspective(fov),
            "orthographic" => lasgun::Camera::orthographic(scale),
            "isometric" => lasgun::Camera::orthographic(scale), // same thing
            _ => lasgun::Camera::perspective(fov) // TODO: Panic instead?
        };
        let origin = utils::to_vec3f(settings.origin());
        let look = utils::to_vec3f(settings.look());
        let up = utils::to_vec3f(settings.up());
        camera.look_at(origin, look, up);
        camera.set_supersampling(settings.supersampling().unwrap_or(0));
        camera.set_aperture_radius(settings.aperture().unwrap_or(0.));
        Camera(camera)
    }
}

#[wasm_bindgen]
pub struct Scene(lasgun::Scene); impl Native for Scene {
    type Output = lasgun::Scene;
    #[inline] fn into_native(self) -> Self::Output { self.0 }
    #[inline] fn as_native(&self) -> &Self::Output { &self.0 }
    #[inline] fn as_native_mut(&mut self) -> &mut Self::Output { &mut self.0 }
}

#[wasm_bindgen]
impl Scene {
    pub fn new(settings: &SceneSettings) -> Scene {
        let ambient = if let Some(array) = settings.ambient() {
            utils::to_vec3f(array)
        } else {
            [0.; 3]
        };
        let mut scene = lasgun::Scene::new();
        scene.set_ambient_light(ambient);
        scene.set_mesh_smoothing(settings.smoothing().unwrap_or(true));
        Scene(scene)
    }

    pub fn set_root(&mut self, content: Aggregate) {
        self.0.set_root(content.into_native())
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.0.set_camera(camera.into_native());
    }

    pub fn set_solid_background(&mut self, color: Box<[JsValue]>) {
        let color = utils::to_vec3f(color);
        self.0.set_solid_background(color)
    }

    pub fn set_radial_background(&mut self, background: RadialBackground) {
        let inner = utils::to_vec3f(background.inner());
        let outer = utils::to_vec3f(background.outer());
        let scale = background.scale().unwrap_or(0.5);

        self.0.set_radial_background(inner, outer, scale)
    }

    pub fn set_supersampling(&mut self, base: u8) {
        self.0.camera.set_supersampling(base)
    }

    pub fn add_obj(&mut self, obj: &str) -> ObjRef {
        ObjRef(self.0.parse_obj(obj).unwrap())
    }

    pub fn add_point_light(&mut self, settings: &PointLight) {
        let position = utils::to_vec3f(settings.position());
        let intensity = utils::to_vec3f(settings.intensity());
        let falloff = utils::to_vec3f(settings.falloff());
        self.0.add_point_light(position, intensity, falloff);
    }
}

#[wasm_bindgen]
pub struct Aggregate(lasgun::scene::Aggregate); impl Native for Aggregate {
    type Output = lasgun::scene::Aggregate;
    #[inline] fn into_native(self) -> Self::Output { self.0 }
    #[inline] fn as_native(&self) -> &Self::Output { &self.0 }
    #[inline] fn as_native_mut(&mut self) -> &mut Self::Output { &mut self.0 }
}

#[wasm_bindgen]
impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate(lasgun::scene::Aggregate::new())
    }

    pub fn add_group(&mut self, node: Aggregate) {
        self.0.add_group(node.into_native())
    }

    pub fn add_sphere(&mut self, sphere: &Sphere, material: &Material) {
        let origin = utils::to_vec3f(sphere.origin());
        let radius = sphere.radius();
        self.0.add_sphere(origin, radius, *material.as_native())
    }

    pub fn add_cube(&mut self, cube: &Cube, material: &Material) {
        let origin = utils::to_vec3f(cube.origin());
        let dim = cube.dim();
        self.0.add_cube(origin, dim, *material.as_native())
    }

    pub fn add_box(&mut self, cuboid: &Cuboid, material: &Material) {
        let start = utils::to_vec3f(cuboid.start());
        let end = utils::to_vec3f(cuboid.end());
        self.0.add_box(start, end, *material.as_native())
    }

    // TODO: Implement add_obj and add_obj_of, which takes a material
    pub fn add_obj(&mut self, mesh: &ObjRef, material: &Material) {
        self.0.add_obj_of(mesh.into_native(), *material.as_native())
    }

    /// Translate by the given delta values, x y and z
    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.0.translate([dx, dy, dz]);
    }

    /// Scale in the given directions
    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.0.scale(x, y, z);
    }

    pub fn rotate_x(&mut self, theta: f64) {
        self.0.rotate_x(theta);
    }

    pub fn rotate_y(&mut self, theta: f64) {
        self.0.rotate_y(theta);
    }

    pub fn rotate_z(&mut self, theta: f64) {
        self.0.rotate_z(theta);
    }

    pub fn rotate(&mut self, theta: f64, axis: Box<[JsValue]>) {
        self.0.rotate(theta, utils::to_vec3f(axis));
    }
}


/// Rendering Accelerator primitive. Implementation includes unsafe lifetime
/// extension, as bindgen does not yet support lifetime constraints. The program
/// will access invalid memory if the instance is accessed after its referenced
/// scene is moved/dropped.
#[wasm_bindgen]
pub struct Accel(lasgun::Accel<'static>); impl Native for Accel {
    type Output = lasgun::Accel<'static>;
    #[inline] fn into_native(self) -> Self::Output { self.0 }
    #[inline] fn as_native(&self) -> &Self::Output { &self.0 }
    #[inline] fn as_native_mut(&mut self) -> &mut Self::Output { &mut self.0 }
}

#[wasm_bindgen]
impl Accel {
    pub fn from(scene: &Scene) -> Accel {
        // This is necessary because wasm_bindgen does not yet support lifetimes
        let scene = unsafe { mem::transmute::<&Scene, &'static Scene>(scene) };
        Accel(lasgun::Accel::from(scene.as_native()))
    }
}

/// Captureable film
#[wasm_bindgen]
pub struct Film {
    pub w: u32,
    pub h: u32,
    winv: f64,
    hinv: f64,
    aspect: f64,
    output: Vec<Pixel>
}

#[wasm_bindgen]
impl Film {
    pub fn new(width: u32, height: u32) -> Film {
        Film {
            w: width,
            h: height,
            winv: 1. / width as f64,
            hinv: 1. / height as f64,
            aspect: width as f64 / height as f64,
            output: vec![[0; 4]; width as usize * height as usize]
        }
    }

    pub fn size(&self) -> usize {
        self.output.len()
    }

    pub fn data_ptr(&self) -> *const u8 {
        unsafe { mem::transmute(self.output[..].as_ptr()) }
    }
}

impl Index<usize> for Film {
    type Output = Pixel;
    #[inline] fn index(&self, at: usize) -> &Self::Output { &self.output[at] }
}

impl IndexMut<usize> for Film {
    #[inline] fn index_mut(&mut self, at: usize) -> &mut Self::Output { &mut self.output[at] }
}

impl lasgun::Img for Film {
    #[inline] fn w(&self) -> u32 { self.w }
    #[inline] fn h(&self) -> u32 { self.h }
    #[inline] fn winv(&self) -> f64 { self.winv }
    #[inline] fn hinv(&self) -> f64 { self.hinv }
    #[inline] fn aspect(&self) -> f64 { self.aspect }
}

// Lasgun-exposed material
#[wasm_bindgen]
pub struct Material(lasgun::Material); impl Native for Material {
    type Output = lasgun::Material;
    #[inline] fn into_native(self) -> Self::Output { self.0 }
    #[inline] fn as_native(&self) -> &Self::Output { &self.0 }
    #[inline] fn as_native_mut(&mut self) -> &mut Self::Output { &mut self.0 }
}

#[wasm_bindgen]
impl Material {
    pub fn plastic(settings: &Plastic) -> Material {
        let kd = utils::to_vec3f(settings.kd());
        let ks = utils::to_vec3f(settings.ks());
        let roughness = settings.roughness().unwrap_or(0.0);
        Material(lasgun::Material::plastic(kd, ks, roughness))
    }

    pub fn matte(settings: &Matte) -> Material {
        let kd = utils::to_vec3f(settings.kd());
        let sigma = settings.sigma().unwrap_or(0.0);
        Material(lasgun::Material::matte(kd, sigma))
    }

    pub fn metal(settings: &Metal) -> Material {
        let eta = utils::to_vec3f(settings.eta());
        let k = utils::to_vec3f(settings.k());
        let (mut u_roughness, mut v_roughness) = (0.0, 0.0);
        if let Some(roughness) = settings.roughness() {
            u_roughness = roughness;
            v_roughness = roughness;
        }

        if let Some(u) = settings.u_roughness() { u_roughness = u }
        if let Some(v) = settings.v_roughness() { v_roughness = v }

        Material(lasgun::Material::metal(eta, k, u_roughness, v_roughness))
    }

    pub fn mirror(settings: &Mirror) -> Material {
        let kr = if let Some(value) = settings.kr() {
            utils::to_vec3f(value)
        } else {
            [1.0, 1.0, 1.0]
        };
        Material(lasgun::Material::mirror(kr))
    }

    pub fn glass(settings: &Glass) -> Material {
        let kr = if let Some(val) = settings.kr()
            { utils::to_vec3f(val) } else { [1.0, 1.0, 1.0] };
        let kt = if let Some(val) = settings.kt()
            { utils::to_vec3f(val) } else { [1.0, 1.0, 1.0] };
        let eta = if let Some(val) = settings.eta() { val } else { 1.5 };
        Material(lasgun::Material::glass(kr, kt, eta))
    }
}
