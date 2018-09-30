use ::lasgun::{ scene::{Scene, Options}, output };

mod meshes;
use self::meshes::smstdodeca;

fn main() { output::render(&simple(), "simple.png"); }

fn simple() -> Scene {
    let options = Options {
        eye: [0.0, 0.0, 800.0],
        view: [0.0, 0.0, -800.0],
        up: [0.0, 1.0, 0.0],
        ambient: [0.3, 0.3, 0.3],
        width: 512,
        height: 512,
        fov: 50.0,
        supersampling: 2,
        threads: 0
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);

    // Add materials to the scene
    let mat0 = scene.add_phong_material([0.7, 1.0, 0.7], [0.5, 0.7, 0.5], 25);
    let mat1 = scene.add_phong_material([0.5, 0.5, 0.5], [0.5, 0.7, 0.5], 25);
    let mat2 = scene.add_phong_material([1.0, 0.6, 0.1], [0.5, 0.7, 0.5], 25);
    let mat3 = scene.add_phong_material([0.7, 0.6, 1.0], [0.5, 0.4, 0.8], 25);

    // Set up scene lights
    scene.add_point_light([-100.0, 150.0, 400.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    scene.add_point_light([400.0, 100.0, 150.0], [0.7, 0.0, 0.7], [1.0, 0.0, 0.0]);

    // Make and aggregate some spheres
    scene.contents.add_sphere([0.0, 0.0, -400.0], 100.0, mat0);
    scene.contents.add_sphere([200.0, 50.0, -100.0], 150.0, mat0);
    scene.contents.add_sphere([0.0, -1200.0, -500.0], 1000.0, mat1);
    scene.contents.add_sphere([-100.0, 25.0, -300.0], 50.0, mat2);
    scene.contents.add_sphere([0.0, 100.0, -250.0], 25.0, mat0);
    scene.contents.add_cube([-200.0, -125.0, 0.0], 100.0, mat3);

    let (verts, faces) = smstdodeca::smstdodeca();
    scene.contents.add_mesh(verts, faces, mat2);
    scene
}
