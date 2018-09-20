extern crate lasgun;
extern crate cfg_if;
extern crate wasm_bindgen;

mod utils;

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

    /// Ducktype for Triangle Mesh settings
    pub type Mesh;
    #[wasm_bindgen(method, getter, structural)]
    pub fn vertices(this: &Mesh) -> Box<[f32]>;
    #[wasm_bindgen(method, getter, structural)]
    pub fn faces(this: &Mesh) -> Box<[u32]>;


    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn scene(settings: &Settings) -> Scene {
    Scene::new(settings)
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct MaterialRef(lasgun::scene::MaterialRef);

impl MaterialRef {
    #[inline]
    pub fn native(&self) -> lasgun::scene::MaterialRef {
        self.0
    }

    #[inline]
    pub fn as_native(self) -> lasgun::scene::MaterialRef {
        self.0
    }
}

#[wasm_bindgen]
pub struct Scene {
    data: lasgun::Scene
}

#[wasm_bindgen]
impl Scene {
    pub fn new(settings: &Settings) -> Scene {
        let ambient = if let Some(array) = settings.ambient() {
            utils::to_vec3f(array)
        } else {
            [0.0; 3]
        };

        let options = lasgun::Options {
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

    pub fn set_contents(&mut self, content: Aggregate) {
        self.data.set_contents(content.as_native_aggregate())
    }

    pub fn add_phong_material(&mut self, settings: &Phong) -> MaterialRef {
        let kd = utils::to_vec3f(settings.kd());
        let ks = utils::to_vec3f(settings.ks());
        let shininess = settings.shininess();
        MaterialRef(self.data.add_phong_material(kd, ks, shininess))
    }

    pub fn add_point_light(&mut self, settings: &PointLight) {
        let position = utils::to_vec3f(settings.position());
        let intensity = utils::to_vec3f(settings.intensity());
        let falloff = utils::to_vec3f(settings.falloff());
        self.data.add_point_light(position, intensity, falloff);
    }

}

#[wasm_bindgen]
pub struct Aggregate {
    data: lasgun::Aggregate
}

#[wasm_bindgen]
impl Aggregate {
    pub fn new() -> Aggregate {
        Aggregate { data: lasgun::Aggregate::new() }
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

    pub fn add_mesh(&mut self, mesh: &Mesh, material: &MaterialRef) {
        let vertices = mesh.vertices();
        let faces = mesh.faces();
        self.data.add_mesh(vertices, faces, material.native())
    }
}

impl Aggregate {
    pub fn as_native_aggregate(self) -> lasgun::Aggregate {
        self.data
    }
}
