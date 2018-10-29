use ::lasgun::{ scene::{Aggregate, Scene, Options}, output };

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
        supersampling: 2,
        threads: 0,
        recursion: 0
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);
    let mat0 = scene.add_phong_material([1.0, 0.5, 1.0], [0.5, 0.5, 0.5], 25);

    let bunny = scene.load_mesh_at(meshes::path("dragon").as_path()).unwrap();

    scene.add_point_light([0.0, 2.0, 3.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    scene.set_radial_background([237, 222, 93], [240, 152, 25]);

    let mut contents = Aggregate::new();
    contents.scale(3.0, 3.0, 3.0);
    contents.rotate_y(90.0);
    // scene.contents.add_sphere([0.0, 0.0, 0.0], 1.0, mat0);
    contents.add_mesh(bunny, mat0);
    scene.root.add_group(contents);

    scene
}

fn main() { output::render(&playground(), "playground.png"); }
