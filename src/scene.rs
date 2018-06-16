use space::{ Vector, Point, Color };

use light::Light;
use primitive::Primitive;
use shape::Intersection;

pub struct Scene {
    pub aggregate: Box<Primitive>,

    pub eye: Point,
    pub view: Vector,
    pub up: Vector,

    pub ambient: Color, // ambient lighting
    pub lights: Vec<Box<Light>> // point-light sources in the scene
}

impl Scene {
    // Find the intersection for the ray defined by the given eye/direction If no intersection is
    // found, returns an intersetion at infinity with the root aggregate primitive.
    pub fn intersect(&self, e: &Point, d: &Vector) -> (Intersection, &Primitive) {
        self.aggregate.intersect(e, d)
    }
}
