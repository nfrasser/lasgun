mod utils;

use std::mem;
use lasgun;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    /// Duck-type for Scene settings representation in JavaScript
    /// For Objects that have the form
    /// {
    ///     eye: [number, number, number],
    ///     view: [number, number, number],
    ///     up: [number, number, number],
    ///     ambient?: [number, number, number],
    ///     width: number, // u16
    ///     height: number, // u16
    ///     fov: number,
    ///     sampling?: number // u8
    ///     threads?: number // u8
    /// }
    pub type Settings;
    #[wasm_bindgen(method, getter, structural)]
    pub fn eye(this: &Settings) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn view(this: &Settings) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn up(this: &Settings) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn ambient(this: &Settings) -> Option<Box<[JsValue]>>; // Optional vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn width(this: &Settings) -> u16;
    #[wasm_bindgen(method, getter, structural)]
    pub fn height(this: &Settings) -> u16;
    #[wasm_bindgen(method, getter, structural)]
    pub fn fov(this: &Settings) -> f64;
    #[wasm_bindgen(method, getter, structural)]
    pub fn sampling(this: &Settings) -> Option<u8>;
    #[wasm_bindgen(method, getter, structural)]
    pub fn threads(this: &Settings) -> Option<u8>;

    /// Duck-type Phong material settings
    /// For JavaScript objects that have the form
    /// {
    ///     kd: [number, number, number],
    ///     ks: [number, number, number],
    ///     shininess: number // (i32)
    /// }
    pub type Phong;
    #[wasm_bindgen(method, getter, structural)]
    pub fn kd(this: &Phong) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn ks(this: &Phong) -> Box<[JsValue]>; // Vector
    #[wasm_bindgen(method, getter, structural)]
    pub fn shininess(this: &Phong) -> i32;

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

    fn alert(s: &str);
}

/// Alias for the scene constructor
#[wasm_bindgen]
pub fn scene(settings: &Settings) -> Scene {
    Scene::new(settings)
}

/// Generate a new film for the given scene
/// Alias for the film constructor
#[wasm_bindgen]
pub fn film(scene: &Scene) -> Film {
    Film::new(scene)
}

/// Capture the whole scene onto the given film
#[wasm_bindgen]
pub fn capture(scene: &Scene, film: &mut Film) {
    lasgun::capture(scene.native(), film.native_mut())
}

/// Capture just one of the 16×16 portions of the scene onto the given hunk of
/// film. Used to progressively stream scene data back to the client. In
/// addition to writing data, writes the target start and end x/y coordinates
/// into the hunk.
#[wasm_bindgen]
pub fn capture_hunk(i: u32, scene: &Scene, root: &Accel, hunk: &mut Hunk) {
    let (hhunks, _) = scene.hunk_dims();
    hunk.x = (i % hhunks as u32 * 16) as u16;
    hunk.y = (i / hhunks as u32 * 16) as u16;
    lasgun::capture_hunk(hunk.x, hunk.y, scene.native(), root.native(), hunk.data_mut())
}

/// The number of 16x16 hunks that make up this scene
#[wasm_bindgen]
pub fn hunk_count(scene: &Scene) -> u32 {
    let (hcount, vcount) = scene.hunk_dims();
    (hcount as u32) * (vcount as u32)
}

// Reference to a material in a scene
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct MaterialRef(lasgun::scene::MaterialRef); impl MaterialRef {
    #[inline] pub fn native(&self) -> lasgun::scene::MaterialRef { self.0 }
    #[inline] pub fn as_native(self) -> lasgun::scene::MaterialRef { self.0 }
}

// Triangle mesh reference in a scene
#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct ObjRef(lasgun::scene::ObjRef); impl ObjRef {
    #[inline] pub fn native(&self) -> lasgun::scene::ObjRef { self.0 }
    #[inline] pub fn as_native(self) -> lasgun::scene::ObjRef { self.0 }
}

/// A 16×16 view into some scene, starting at the given x/y coordinates
/// (from the top-left)
#[wasm_bindgen]
pub struct Hunk {
    /// Staring x coordinate
    pub x: u16,
    /// Staring y coordinate
    pub y: u16,

    data: lasgun::FilmDataHunk
}

#[wasm_bindgen]
impl Hunk {
    /// Create a new empty hunk for the given scene with the given index
    pub fn new() -> Hunk {
        Hunk { x: 0, y: 0, data: unsafe { mem::uninitialized() } }
    }

    /// Get a pointer to the pixel data in the hunk for use in JavaScript
    pub fn as_ptr(&self) -> *const u8 {
        &self.data as *const u8
    }
}

impl Hunk {
    #[inline]
    pub fn data_mut(&mut self) -> &mut lasgun::FilmDataHunk {
        &mut self.data
    }
}

#[wasm_bindgen]
pub struct Scene {
    data: lasgun::scene::Scene
}

#[wasm_bindgen]
impl Scene {
    pub fn new(settings: &Settings) -> Scene {
        let ambient = if let Some(array) = settings.ambient() {
            utils::to_vec3f(array)
        } else {
            [0.0; 3]
        };

        let options = lasgun::scene::Options {
            eye: utils::to_vec3f(settings.eye()),
            view: utils::to_vec3f(settings.view()),
            up: utils::to_vec3f(settings.up()),
            ambient: ambient,
            width: settings.width(),
            height: settings.height(),
            fov: settings.fov(),
            supersampling: settings.sampling().unwrap_or(0),
            threads: settings.threads().unwrap_or(0)
        };

        Scene { data: lasgun::Scene::new(options) }
    }

    pub fn width(&self) -> u16 {
        self.data.options.width
    }

    pub fn height(&self) -> u16 {
        self.data.options.height
    }

    pub fn set_root(&mut self, content: Aggregate) {
        self.data.set_root(content.as_native_aggregate())
    }

    pub fn add_phong_material(&mut self, settings: &Phong) -> MaterialRef {
        let kd = utils::to_vec3f(settings.kd());
        let ks = utils::to_vec3f(settings.ks());
        let shininess = settings.shininess();
        MaterialRef(self.data.add_phong_material(kd, ks, shininess))
    }

    pub fn add_obj(&mut self, obj: &str) -> ObjRef {
        ObjRef(self.data.add_mesh_from(obj).unwrap())
    }

    pub fn add_point_light(&mut self, settings: &PointLight) {
        let position = utils::to_vec3f(settings.position());
        let intensity = utils::to_vec3f(settings.intensity());
        let falloff = utils::to_vec3f(settings.falloff());
        self.data.add_point_light(position, intensity, falloff);
    }
}

impl Scene {
    pub fn blank() -> Scene {
        let options = lasgun::scene::Options {
            eye: [0.0, 0.0, 0.0],
            view: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],
            ambient: [0.0, 0.0, 0.0],
            width: 0,
            height: 0,
            fov: 0.0,
            supersampling: 0,
            threads: 0
        };

        Scene { data: lasgun::Scene::new(options) }
    }

    #[inline]
    pub fn native(&self) -> &lasgun::scene::Scene {
        &self.data
    }

    /// Returns the number of width-wise and height-wise number of hunks for
    /// this scene
    #[inline]
    pub fn hunk_dims(&self) -> (u16, u16) {
        let (width, height) = (self.width(), self.height());
        (
            width / 16 + (if width % 16 == 0 { 0 } else { 1 }),
            height / 16 + (if height % 16 == 0 { 0 } else { 1 })
        )
    }
}

#[wasm_bindgen]
pub struct Aggregate { data: lasgun::scene::Aggregate }

#[wasm_bindgen]
impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate { data: lasgun::scene::Aggregate::new() }
    }

    pub fn add_group(&mut self, node: Aggregate) {
        self.data.add_group(node.data)
    }

    pub fn add_sphere(&mut self, sphere: &Sphere, material: &MaterialRef) {
        let origin = utils::to_vec3f(sphere.origin());
        let radius = sphere.radius();
        self.data.add_sphere(origin, radius, material.native())
    }

    pub fn add_cube(&mut self, cube: &Cube, material: &MaterialRef) {
        let origin = utils::to_vec3f(cube.origin());
        let dim = cube.dim();
        self.data.add_cube(origin, dim, material.native())
    }

    pub fn add_box(&mut self, cuboid: &Cuboid, material: &MaterialRef) {
        let start = utils::to_vec3f(cuboid.start());
        let end = utils::to_vec3f(cuboid.end());
        self.data.add_box(start, end, material.native())
    }

    pub fn add_mesh(&mut self, mesh: &ObjRef, material: &MaterialRef) {
        self.data.add_mesh(mesh.native(), material.native())
    }

    /// Translate by the given delta values, x y and z
    pub fn translate(&mut self, dx: f64, dy: f64, dz: f64) {
        self.data.translate([dx, dy, dz]);
    }

    /// Scale in the given directions
    pub fn scale(&mut self, x: f64, y: f64, z: f64) {
        self.data.scale(x, y, z);
    }

    pub fn rotate_x(&mut self, theta: f64) {
        self.data.rotate_x(theta);
    }

    pub fn rotate_y(&mut self, theta: f64) {
        self.data.rotate_y(theta);
    }

    pub fn rotate_z(&mut self, theta: f64) {
        self.data.rotate_z(theta);
    }

    pub fn rotate(&mut self, theta: f64, axis: Box<[JsValue]>) {
        self.data.rotate(theta, utils::to_vec3f(axis));
    }
}

impl Aggregate {
    #[inline]
    pub fn as_native_aggregate(self) -> lasgun::scene::Aggregate {
        self.data
    }
}

/// Rendering Accelerator primitive. Implementation includes unsafe lifetime
/// extension, as bindgen does not yet support lifetime constraints. The program
/// will access invalid memory if the instance is accessed after its referenced
/// scene is moved/dropped.
#[wasm_bindgen]
pub struct Accel(lasgun::Accel<'static>); impl Accel {
    pub fn native(&self) -> &lasgun::Accel<'static> { &self.0 }
    pub fn as_native(self) -> lasgun::Accel<'static> { self.0 }
}

#[wasm_bindgen]
impl Accel {
    pub fn from(scene: &Scene) -> Accel {
        // This is necessary because wasm_bindgen does not yet support lifetimes
        let scene = unsafe { mem::transmute::<&Scene, &'static Scene>(scene) };
        Accel(lasgun::Accel::from(&scene.data))
    }
}

/// Captureable film
#[wasm_bindgen]
pub struct Film(lasgun::Film); impl Film {
    pub fn native(&self) -> &lasgun::Film { &self.0 }
    pub fn native_mut(&mut self) -> &mut lasgun::Film { &mut self.0 }
}

#[wasm_bindgen]
impl Film {
    pub fn new(scene: &Scene) -> Film {
        let (width, height) = (
            scene.native().options.width,
            scene.native().options.height
        );
        Film(lasgun::Film::new(width, height))
    }

    /// Get a Uint8Array of pixels for display in a browser environment
    /// Each set of 4 bytes represents an RGBA pixel
    pub fn pixels(&self) -> Box<[u8]> {
        self.0.data.as_slice().to_vec().into_boxed_slice()
    }

    /// Get a pointer to the first pixel in the data set
    pub fn data(&self) -> *const u8 {
        unsafe { mem::transmute(self.0.data.raw_pixels()) }
    }

    /// How many bytes there are in the data pointer
    pub fn size(&self) -> usize {
        4
        * self.0.width as usize
        * self.0.height as usize
    }
}
