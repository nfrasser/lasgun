extern crate image;
extern crate lasgun;

use lasgun::{
    scene, Scene, Color, Point, Vector,
    primitive::aggregate::Aggregate
};

mod common;
use common::output;

fn main() {
    let options = scene::Options {
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

    let scene = Scene::new(options);
    output::render(&scene, "image.png")
}
