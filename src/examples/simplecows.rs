use ::lasgun::{ aggregate::Aggregate, scene::{Scene, Options}, output};

mod meshes;

fn main() { output::render(&simplecows(), "simplecows.png"); }

fn simplecows() -> Scene {
    let mut scene = Scene::new(Options {
        eye: [0.0, 2.0, 30.0],
        view: [0.0, 0.0, -1.0],
        up: [0.0, 1.0, 0.0],
        fov: 50.0,
        ambient: [0.4, 0.4, 0.4],
        width: 512,
        height: 512,
        supersampling: 2,
        threads: 0
    });

    scene.add_point_light([200.0, 202.0, 430.0], [0.8, 0.8, 0.8], [1.0, 0.0, 0.0]);

    let stone = scene.add_phong_material([0.8, 0.7, 0.7], [0.0, 0.0, 0.0], 0);
    let grass = scene.add_phong_material([0.1, 0.7, 0.1], [0.0, 0.0, 0.0], 0);
    let hide = scene.add_phong_material([0.84, 0.6, 0.53], [0.3, 0.3, 0.3], 20);

    // The Floor
    let mut plane = Aggregate::new();
    plane.scale(30.0, 30.0, 30.0);
    let (vertices, faces) = meshes::plane();
    plane.add_mesh(vertices, faces, grass);
    scene.contents.add_aggregate(plane);

    // Central altar
    let mut buckyball = Aggregate::new();
    buckyball.scale(1.5, 1.5, 1.5);
    let (vertices, faces) = meshes::buckyball();
    buckyball.add_mesh(vertices, faces, stone);
    scene.contents.add_aggregate(buckyball);

    // Ring of arches
    for i in 1..=6 {

        let mut p1 = Aggregate::new();
        p1.add_cube([0.0, 0.0, 0.0], 1.0, stone);
        p1.translate([-2.4, 0.0, -0.4]).scale(0.8, 4.0, 0.8);

        let mut p2 = Aggregate::new();
        p2.add_cube([0.0, 0.0, 0.0], 1.0, stone);
        p2.translate([1.6, 0.0, -0.4]).scale(0.8, 4.0, 0.8);

        let mut s = Aggregate::new();
        s.add_sphere([0.0, 0.0, 0.0], 1.0, stone);
        s.scale(4.0, 0.6, 0.6).translate([0.0, 4.0, 0.0]);

        let mut arc = Aggregate::new();
        arc.add_aggregate(p1);
        arc.add_aggregate(p2);
        arc.add_aggregate(s);

        arc.translate([0.0, 0.0, -10.0]);
        arc.rotate_y(((i-1) * 60) as f64);

        scene.contents.add_aggregate(arc)
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
            ([-0.7, -0.7, 0.7], 0.0),      // rrleg
        ].iter() {
            cow.add_sphere(*center, *radius, hide);
        }

        scene.contents.add_aggregate(cow)
    }

    scene.contents.rotate_x(-23.0);
    scene
}
