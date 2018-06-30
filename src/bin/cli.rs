extern crate image;
extern crate lasgun;

use lasgun::{
    scene, Scene, Color, Point, Vector,
    primitive::aggregate::Aggregate
};

mod common::output;

fn main() {
    let scene = Scene::new(scene::Options {
        dimensions: (256, 256),
        content: Box::new(Aggregate::new(vec![])),
        lights: vec![],
        ambient: Color::zeros(),
        eye: Point::new(0.0, 0.0, 0.0),
        view: Vector::zeros(),
        up: Vector::zeros(),
        fov: 50.0,
        supersampling: 1
    };
    output::render(&scene, "image.png")
}
