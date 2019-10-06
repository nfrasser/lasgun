use ::lasgun::{ scene::{Scene, Options}, Material, output };

mod meshes;

fn playground() -> Scene {
    let options = Options {
        eye:  [0.0, 1.0, 4.0],
        view: [-0.1, 0.0, -1.0],
        up: [0.0, 1.0, 0.0],
        ambient: [0.1, 0.1, 0.1],
        width: 512,
        height: 512,
        fov: 60.0,
        smoothing: true,
        supersampling: 2,
        threads: 0
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);
    let mat0 = Material::metal([0.9, 0.1, 0.9], [0.7, 1.0, 0.7], 0.25, 0.25);

    let bunny = scene.load_obj(meshes::path("bunny").as_path()).unwrap();

    scene.add_point_light([0.0, 2.0, 3.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    scene.set_radial_background([0.93, 0.87, 0.36], [0.94, 0.6, 0.1]);

    // scene.contents.add_sphere([0.0, 0.0, 0.0], 1.0, mat0);
    scene.root.add_obj_of(bunny, mat0);

    scene
}

fn main() { output::render(&playground(), "playground.png"); }
