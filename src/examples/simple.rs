extern crate image;
extern crate lasgun;

mod common;

use std::rc::Rc;
use lasgun::*;
use material::phong::Phong;
use light::{Light, point::PointLight};
use primitive::{aggregate::Aggregate, geometry::Geometry};

use common::output;

fn main() { output::render(&simple(), "simple.png"); }

fn simple() -> Scene {
    // Make materials
    let mat1 = Phong::new([0.7, 1.0, 0.7], [0.5, 0.7, 0.5], 25);
    let mat2 = Phong::new([0.5, 0.5, 0.5], [0.5, 0.7, 0.5], 25);
    let mat3 = Phong::new([1.0, 0.6, 0.1], [0.5, 0.7, 0.5], 25);
    let mat4 = Phong::new([0.7, 0.6, 1.0], [0.5, 0.4, 0.8], 25);
    let (mat1, mat2, mat3, mat4) = (
        Rc::new(mat1), Rc::new(mat2), Rc::new(mat3), Rc::new(mat4)
    );

    // Make and aggregate some spheres
    let s1 = Geometry::sphere([0.0, 0.0, -400.0], 100.0, mat1.clone());
    let s2 = Geometry::sphere([200.0, 50.0, -100.0], 150.0, mat1.clone());
    let s3 = Geometry::sphere([0.0, -1200.0, -500.0], 1000.0, mat2.clone());
    let s4 = Geometry::sphere([-100.0, 25.0, -300.0], 50.0, mat3.clone());
    let s5 = Geometry::sphere([0.0, 100.0, -250.0], 25.0, mat1.clone());
    let b1 = Geometry::cube([-200.0, -125.0, 0.0], 100.0, mat4.clone());
    let aggregate = Aggregate::new(vec![
        Box::new(s1), Box::new(s2), Box::new(s3), Box::new(s4), Box::new(s5),
        Box::new(b1)
    ]);

    // Set up scene lights
    let white_light = PointLight::new([-100.0, 150.0, 400.0], [0.9, 0.9, 0.9], [1.0, 0.0, 0.0]);
    let orange_light = PointLight::new([400.0, 100.0, 150.0], [0.7, 0.0, 0.7], [1.0, 0.0, 0.0]);
    let lights: Vec<Box<Light>> = vec![
        Box::new(white_light), Box::new(orange_light)
    ];

    // Return the resulting scene
    let options = scene::Options {

        dimensions: (1024, 1024),
        content: Box::new(aggregate),

        eye: Point::new(0.0, 0.0, 800.0),
        view: Vector::new(0.0, 0.0, -800.0),
        up: Vector::new(0.0, 1.0, 0.0),

        fov: 50.0,
        ambient: Color::new(0.3, 0.3, 0.3),
        lights,

        supersampling: 10
    };

    Scene::new(options)
}
