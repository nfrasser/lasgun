use ::lasgun::{scene::{Scene, Aggregate}, Material, output};

mod meshes;

fn simplecows() -> Scene {
    let mut scene = Scene::new();
    scene.set_ambient_light([0.2, 0.2, 0.2]);
    scene.set_radial_background([0.85, 0.82, 0.6], [0.69, 0.85, 0.73], 0.5);

    let camera = scene.set_perspective_camera(50.);
    camera.look_at([0., 2., 30.], [0., 2., 29.], [0., 1., 0.]);
    camera.set_supersampling(2);

    // Lights
    scene.add_point_light([200.0, 202.0, 430.0], [0.8, 0.8, 0.8], [1.0, 0.0, 0.0]);

    // Materials
    let stone = Material::metal([0.0, 0.0, 0.0], [0.7, 0.7, 0.7], 0.5, 0.5);
    let grass = Material::plastic([0.1, 0.7, 0.1], [0.0, 0.0, 0.0], 0.0);
    let hide = Material::plastic([0.84, 0.6, 0.53], [0.3, 0.3, 0.3], 0.2);

    // Meshes
    let planemesh = scene.load_obj(meshes::path("plane").as_path()).unwrap();
    let buckyballmesh = scene.load_obj(meshes::path("buckyball").as_path()).unwrap();

    // The Floor
    let mut plane = Aggregate::new();
    plane.scale(30.0, 30.0, 30.0);
    plane.add_obj_of(planemesh, grass);
    scene.root.add_group(plane);

    // Central altar
    let mut buckyball = Aggregate::new();
    buckyball.scale(1.5, 1.5, 1.5);
    buckyball.add_obj_of(buckyballmesh, stone);
    scene.root.add_group(buckyball);

    // Ring of arches
    for i in 1..=6 {

        let mut p1 = Aggregate::new();
        p1.add_cube([0.0, 0.0, 0.0], 1.0, stone);
        p1.scale(0.8, 4.0, 0.8).translate([-2.4, 0.0, -0.4]);

        let mut p2 = Aggregate::new();
        p2.add_cube([0.0, 0.0, 0.0], 1.0, stone);
        p2.scale(0.8, 4.0, 0.8).translate([1.6, 0.0, -0.4]);

        let mut s = Aggregate::new();
        s.add_sphere([0.0, 0.0, 0.0], 1.0, stone);
        s.scale(4.0, 0.6, 0.6).translate([0.0, 4.0, 0.0]);

        let mut arc = Aggregate::new();
        arc.add_group(p1);
        arc.add_group(p2);
        arc.add_group(s);

        arc.translate([0.0, 0.0, -10.0]);
        arc.rotate_y(((i-1) * 60) as f64);

        scene.root.add_group(arc)
    }

    // Create some simple cows, transforming each one
    for (translation, rotation) in [
        ([1.0, 1.3, 14.0], 20.0),
        ([5.0, 1.3, -11.0], 180.0),
        ([-5.5, 1.3, -3.0], -60.0),
    ].iter() {
        let mut cow = Aggregate::new();
        cow .scale(1.4, 1.4, 1.4)
            .rotate_y(*rotation)
            .translate(*translation);

        for (center, radius) in [
            ([0.0, 0.0, 0.0], 1.0),           // body
            ([0.9, 0.3, 0.0], 0.6),       // head
            ([-0.94, 0.34, 0.0], 0.2),    // tail
            ([0.7, -0.7, -0.7], 0.3),      // lfleg
            ([-0.7, -0.7, -0.7], 0.3),     // lrleg
            ([0.7, -0.7, 0.7], 0.3),       // rfleg
            ([-0.7, -0.7, 0.7], 0.3),      // rrleg
        ].iter() {
            cow.add_sphere(*center, *radius, hide);
        }

        scene.root.add_group(cow)
    }

    scene.root.rotate_x(23.0);
    scene
}

fn main() { output::render(&simplecows(), [512, 512], "simplecows.png"); }
