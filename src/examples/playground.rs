use ::lasgun::{ scene::{Scene, Options}, output };

mod meshes;

fn playground() -> Scene {
    let options = Options {
        eye:  [0.0, 1.0, 4.0],
        view: [-0.1, 0.0, -1.0],
        up: [0.0, 1.0, 0.0],
        ambient: [0.3, 0.3, 0.3],
        width: 512,
        height: 512,
        fov: 60.0,
        supersampling: 0,
        threads: 4
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);
    let mat0 = scene.add_phong_material([0.7, 1.0, 0.7], [0.5, 0.7, 0.5], 25);

    let bunny = scene.add_mesh_at(meshes::path("bunny").as_path()).unwrap();

    scene.add_point_light([0.0, 2.0, 3.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);

    // scene.contents.add_sphere([0.0, 0.0, 0.0], 1.0, mat0);
    scene.root.add_mesh(bunny, mat0);

    scene
}

fn main() { output::render(&playground(), "playground.png"); }
