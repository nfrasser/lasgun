use ::lasgun::{scene::Scene, Material, output};

mod meshes;

fn playground() -> Scene {
    // Initialize a new empty scene with the given options
    let mut scene = Scene::new();
    scene.set_ambient_light([0.1, 0.1, 0.1]);
    scene.set_radial_background([0.93, 0.87, 0.36], [0.94, 0.6, 0.1], 0.8);

    let camera = scene.set_perspective_camera(60.);
    camera.look_at([0., 1., 4.], [-0.1, 1., 3.], [0., 1., 0.]);
    camera.set_supersampling(2);

    let mat0 = Material::metal([0.9, 0.1, 0.9], [0.7, 1.0, 0.7], 0.25, 0.25);

    let bunny = scene.load_obj(meshes::path("bunny").as_path()).unwrap();

    scene.add_point_light([0.0, 2.0, 3.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);

    // scene.contents.add_sphere([0.0, 0.0, 0.0], 1.0, mat0);
    scene.root.add_obj_of(bunny, mat0);

    scene
}

fn main() { output::render(&playground(), [512, 512], "playground.png"); }
