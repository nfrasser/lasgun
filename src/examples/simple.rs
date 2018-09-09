extern crate image;
extern crate lasgun;

mod common;
mod meshes;

use lasgun::*;
use material::{Material, phong::Phong};
use light::{Light, point::PointLight};
use primitive::{aggregate::Aggregate, geometry::Geometry};

use common::output;
use meshes::smstdodeca;

fn main() { output::render(&simple(), "simple.png"); }

fn simple() -> Scene {
    let ambient = Color::new(0.3, 0.3, 0.3);

    let (mat0, mat0i) = (Phong::new([0.7, 1.0, 0.7], [0.5, 0.7, 0.5], 25), 0);
    let (mat1, mat1i) = (Phong::new([0.5, 0.5, 0.5], [0.5, 0.7, 0.5], 25), 1);
    let (mat2, mat2i) = (Phong::new([1.0, 0.6, 0.1], [0.5, 0.7, 0.5], 25), 2);
    let (mat3, mat3i) = (Phong::new([0.7, 0.6, 1.0], [0.5, 0.4, 0.8], 25), 3);

    // Make materials
    let materials: Vec<Box<Material>> = vec![
        Box::new(mat0), Box::new(mat1), Box::new(mat2), Box::new(mat3)
    ];

    // Make and aggregate some spheres
    let s1 = Geometry::sphere([0.0, 0.0, -400.0], 100.0, mat0i);
    let s2 = Geometry::sphere([200.0, 50.0, -100.0], 150.0, mat0i);
    let s3 = Geometry::sphere([0.0, -1200.0, -500.0], 1000.0, mat1i);
    let s4 = Geometry::sphere([-100.0, 25.0, -300.0], 50.0, mat2i);
    let s5 = Geometry::sphere([0.0, 100.0, -250.0], 25.0, mat0i);
    let b1 = Geometry::cube([-200.0, -125.0, 0.0], 100.0, mat3i);

    let steldodec = Geometry::mesh(smstdodeca::smstdodeca(), mat2i);

    let aggregate = Aggregate::new(vec![
        Box::new(s1), Box::new(s2), Box::new(s3), Box::new(s4), Box::new(s5),
        Box::new(b1),
        Box::new(steldodec)
    ], ambient);

    // Set up scene lights
    let white_light = PointLight::new([-100.0, 150.0, 400.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    let orange_light = PointLight::new([400.0, 100.0, 150.0], [0.7, 0.0, 0.7], [1.0, 0.0, 0.0]);
    let lights: Vec<Box<Light>> = vec![
        Box::new(white_light), Box::new(orange_light)
    ];

    // Return the resulting scene
    let options = scene::Options {
        dimensions: (512, 512),
        content: Box::new(aggregate),
        materials,

        eye: Point::new(0.0, 0.0, 800.0),
        view: Vector::new(0.0, 0.0, -800.0),
        up: Vector::new(0.0, 1.0, 0.0),

        fov: 50.0,
        ambient,
        lights,

        supersampling: 2,
        threads: 1
    };

    Scene::new(options)
}
