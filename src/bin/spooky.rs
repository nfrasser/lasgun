use std::path::PathBuf;
use ::lasgun::{ scene::{Scene, Options, Aggregate} };

pub fn build_spooky_scene() -> Scene {
    let options = Options {
        eye: [0.0, 6.0, 10.0],
        view: [0.0, -1.5, -5.0],
        up: [0.0, 1.0, 0.0],

        ambient: [0.3, 0.3, 0.3],

        width: 768,
        height: 768,

        fov: 50.0,
        supersampling: 0,
        threads: 0,
        recursion: 0
    };

    // Initialize a new empty scene with the given options
    let mut scene = Scene::new(options);
    scene.set_radial_background([100, 75, 75], [25, 0, 0]);

    let skull = scene.load_mesh_at(obj_path("skull").as_path()).unwrap();
    let plane = scene.load_mesh_at(obj_path("plane").as_path()).unwrap();

    // Add materials to the scene
    let floor = scene.add_phong_material([0.8, 0.7, 0.7], [0.0, 0.0, 0.0], 0);
    let bone = scene.add_phong_material([0.7, 0.7, 0.5], [0.3, 0.3, 0.3], 20);
    let glass = scene.add_phong_material([0.7, 0.6, 1.0], [0.5, 0.4, 0.8], 25);
    // TODO: Glass

    // Set up scene lights
    scene.add_point_light([-20.0, 15.0, 0.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    scene.add_point_light([40.0, 10.0, 15.0], [1.0, 0.5, 0.0], [1.0, 0.0, 0.0]);

    let mut skull_group = Aggregate::new();
    skull_group.scale(0.5, 0.5, 0.5);
    skull_group.translate([9.0, 0.0, -15.0]);
    skull_group.add_mesh(skull, bone);

    let mut item_group = Aggregate::new();
    item_group.add_group(skull_group);
    // item_group.add_sphere([9.0, 4.0, -15.0], 4.0, glass);
    item_group.add_sphere([-3.0, 1.0, 0.0], 1.0, glass);
    item_group.add_sphere([2.5, 0.5, -2.0], 0.5, glass);
    item_group.add_sphere([0.0, 2.0, -15.0], 2.0, glass);
    item_group.add_sphere([2.5, 1.0, -2.0], 1.0, glass);
    item_group.add_sphere([-1.5, 0.5, -3.0], 0.5, glass);

    let mut floor_group = Aggregate::new();
    floor_group.scale(100.0, 1.0, 100.0);
    floor_group.add_mesh(plane, floor);

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
