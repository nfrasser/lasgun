use ::lasgun::{ scene::{Scene, Options, Aggregate}, output };

mod meshes;

fn main() { output::render(&cornell(), "cornell.png"); }

fn cornell() -> Scene {
    let options = Options {
        eye: [0.0, 0.0, 5.0],
        view: [0.0, 0.0, -5.0],
        up: [0.0, 1.0, 0.0],
        ambient: [0.05, 0.05, 0.05],
        width: 512,
        height: 512,
        fov: 60.0,
        supersampling: 2,
        threads: 0
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);

    // Add materials to the scene
    let mat0 = scene.add_plastic_material([0.7, 0.0, 0.7], [0.5, 0.7, 0.5], 0.25);
    let white = scene.add_plastic_material([1.0, 1.0, 1.0], [0.5, 0.7, 0.5], 0.25);
    let r = scene.add_plastic_material([1.0, 0.0, 0.0], [0.5, 0.7, 0.5], 0.25);
    let g = scene.add_plastic_material([0.0, 1.0, 0.0], [0.5, 0.7, 0.5], 0.25);
    // let b = scene.add_plastic_material([0.0, 0.0, 1.0], [0.5, 0.4, 0.8], 0.25);
    // let glass = scene.add_mirror_material([0.0, 0.0, 0.0]);
    let glass = scene.add_rough_glass_material([1.0, 0.7, 1.0], [0.7, 1.0, 0.7], 1.25, 0.0, 0.0);

    // Instantiate meshes to be shown in the scene
    let plane = scene.add_mesh_at(meshes::path("plane").as_path()).unwrap();

    // Set up scene lights
    scene.add_point_light([0.0, 1.75, 0.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    // scene.add_point_light([0.0, 0.0, 4.0], [0.9, 0.9, 0.9], [1.0, 0.2, 0.1]);

    let mut floor = Aggregate::new();
    floor.scale(2.0, 1.0, 2.0);
    floor.translate([0.0, -2.0, 0.0]);
    floor.add_mesh(plane, white);
    scene.root.add_group(floor);

    let mut ceiling = Aggregate::new();
    ceiling.scale(2.0, 1.0, 2.0);
    ceiling.translate([0.0, 2.0, 0.0]);
    ceiling.add_mesh(plane, white);
    scene.root.add_group(ceiling);

    let mut left = Aggregate::new();
    left.scale(2.0, 1.0, 2.0);
    left.rotate_z(90.0);
    left.translate([-2.0, 0.0, 0.0]);
    left.add_mesh(plane, r);
    scene.root.add_group(left);

    let mut right = Aggregate::new();
    right.scale(2.0, 1.0, 2.0);
    right.rotate_z(90.0);
    right.translate([2.0, 0.0, 0.0]);
    right.add_mesh(plane, g);
    scene.root.add_group(right);

    let mut back = Aggregate::new();
    back.scale(2.0, 1.0, 2.0);
    back.rotate_x(90.0);
    back.translate([0.0, 0.0, -2.0]);
    back.add_mesh(plane, white);
    scene.root.add_group(back);

    // Make and aggregate some spheres
    scene.root.add_sphere([1.0, -1.25, 0.0], 1.0, glass);
    scene.root.add_cube([-2.0, -2.0, 0.0], 1.0, glass);

    scene
}