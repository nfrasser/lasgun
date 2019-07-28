use std::path::PathBuf;
use ::lasgun::{ output, scene::{Scene, Options, Aggregate}, Material };

fn main() { output::render(&spooky(), "spooky.png"); }

fn spooky() -> Scene {
    let options = Options {
        eye: [-5.0, 2.0, 6.0],
        view: [2.0, 0.2, -5.0],
        up: [0.0, 1.0, 0.0],

        ambient: [0.3, 0.3, 0.3],

        width: 768,
        height: 768,

        fov: 50.0,
        supersampling: 3,
        threads: 0
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);
    scene.set_radial_background([0.39, 0.29, 0.29], [0.1, 0.0, 0.0]);

    let skull = scene.load_obj(obj_path("skull").as_path()).unwrap();
    let plane = scene.load_obj(obj_path("plane").as_path()).unwrap();

    // Add materials to the scene
    let floor = Material::plastic([0.8, 0.7, 0.7], [0.0, 0.0, 0.0], 0.0);
    let bone = Material::plastic([0.7, 0.7, 0.5], [0.3, 0.3, 0.3], 0.20);
    let purple = Material::plastic([0.7, 0.6, 1.0], [0.8, 0.8, 0.8], 0.25);
    let glass = Material::glass([0.7, 0.6, 1.0], [0.8, 0.8, 0.8], 1.333);

    // Set up scene lights
    scene.add_point_light([-20.0, 15.0, 0.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    scene.add_point_light([40.0, 10.0, 15.0], [1.0, 0.5, 0.0], [1.0, 0.0, 0.0]);

    let mut skull_group = Aggregate::new();
    skull_group.scale(0.5, 0.5, 0.5);
    skull_group.rotate_y(-60.0);
    skull_group.translate([4.0, 0.5, -4.0]);
    skull_group.add_obj_of(skull, bone);

    let mut item_group = Aggregate::new();
    item_group.add_group(skull_group);
    item_group.add_sphere([4.0, 4.0, -11.0], 4.0, purple);
    item_group.add_cube([-2.5, 0.001, -3.0], 1.75, glass);
    item_group.add_sphere([2.5, 0.5, -2.0], 0.5, glass);
    item_group.add_sphere([0.0, 2.0, -15.0], 2.0, glass);
    item_group.add_sphere([2.5, 1.0, -2.0], 1.0, glass);
    // item_group.add_sphere([-1.5, 3.0, -3.0], 3.0, glass);

    let mut floor_group = Aggregate::new();
    floor_group.scale(100.0, 1.0, 100.0);
    floor_group.add_obj_of(plane, floor);

    // Rotate slightly
    scene.root.rotate_y(10.0);
    scene.root.add_group(item_group);
    scene.root.add_group(floor_group);

    scene
}

fn obj_path(name: &str) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(".");
    path.push("meshes");
    path.push(name.clone());
    path.set_extension("obj");
    path
}
