use space::*;

use light::Light;
use primitive::Primitive;
use shape::Intersection;

pub struct Scene {
    pub content: Box<Primitive>,
    pub dimensions: (u16, u16), // width/height of generated image

    pub eye: Point,
    pub view: Vector,
    pub up: Vector,

    pub fov: f64, // field of view

    pub ambient: Color, // ambient lighting
    pub lights: Vec<Box<Light>> // point-light sources in the scene
}

impl Scene {
    // Find the intersection for the ray defined by the given eye/direction If no intersection is
    // found, returns an intersetion at infinity with the root content primitive.
    pub fn intersect(&self, e: &Point, d: &Direction) -> (Intersection, &Primitive) {
        self.content.intersect(e, d)
    }
}
