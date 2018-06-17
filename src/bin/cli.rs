extern crate image;
extern crate lasgun;

use lasgun::{
    Scene, Color, Point, Vector,
    primitive::aggregate::Aggregate
};

mod output;

fn main() {
    let scene = Scene {
        content: Box::new(Aggregate::new(vec![])),
        dimensions: (256, 256),
        eye: Point::new(0.0, 0.0, 0.0),
        view: Vector::zeros(),
        up: Vector::zeros(),
        fov: 50.0,
        ambient: Color::zeros(),
        lights: vec![]
    };
    output::render(&scene, "image.png")
}
