use ::lasgun::{ scene::Scene, Material, output };

mod meshes;

fn main() { output::render(&simple(), [512, 512], "simple.png"); }

fn simple() -> Scene {

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new();
    scene.set_ambient_light([0.2, 0.2, 0.2]);
    scene.set_radial_background([0.26, 0.78, 0.67], [0.1, 0.09, 0.33], 0.5);

    let camera = scene.set_perspective_camera(45.);
    camera.look_at([25., 0., 800.], [25., 0., 0.], [0., 1., 0.]);
    camera.set_supersampling(2);

    // Add materials to the scene
    let mat0 = Material::plastic([0.7, 1.0, 0.7], [0.5, 0.7, 0.5], 0.25);
    let mat1 = Material::plastic([0.5, 0.5, 0.5], [0.5, 0.7, 0.5], 0.25);
    let mat2 = Material::plastic([1.0, 0.6, 0.1], [0.5, 0.7, 0.5], 0.25);
    let mat3 = Material::plastic([0.7, 0.6, 1.0], [0.5, 0.4, 0.8], 0.25);

    // Instantiate meshes to be shown in the scene
    let smstdodeca = scene.load_obj(meshes::path("smstdodeca").as_path()).unwrap();

    // Set up scene lights
    scene.add_point_light([-100.0, 150.0, 400.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    scene.add_point_light([400.0, 100.0, 150.0], [0.7, 0.0, 0.7], [1.0, 0.0, 0.0]);

    // Make and aggregate some spheres
    scene.root.add_sphere([0.0, 0.0, -400.0], 100.0, mat0);
    scene.root.add_sphere([200.0, 50.0, -100.0], 150.0, mat0);
    scene.root.add_sphere([0.0, -1200.0, -500.0], 1000.0, mat1);
    scene.root.add_sphere([-100.0, 25.0, -300.0], 50.0, mat2);
    scene.root.add_sphere([0.0, 100.0, -250.0], 25.0, mat0);
    scene.root.add_cube([-200.0, -125.0, 0.0], 100.0, mat3);
    scene.root.add_obj_of(smstdodeca, mat2);

    scene
}
