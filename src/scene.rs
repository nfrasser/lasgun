use std::f64;
use rand::prelude::*;
use rand::distributions::{Uniform};

use space::*;
use light::Light;
use primitive::Primitive;
use shape::Intersection;


/**
    Description of the world to render
*/
pub struct Scene {
    /**
    The position of the eye/camera in the scene
    */
    pub eye: Point,

    /**
    The direction in which the view vector points.
    Its magnitude is the distance to the focal plane
    */
    pub view: Vector,

    /**
    The orientation of the eye in the scene
    */
    pub up: Vector,

    /**
    Auxilary vectory, orthogonal to the up and view vectors
    */
    pub aux: Vector,

    /**
    Half the distance between two ray intersections points on the focal plane. Primary rays with
    supersampling enabled will sample points around the original ray / intersection. The maximum
    distance of the sample ray interesection points on the focal plane from the original point of
    intersetion will be no larger than this number.
    */
    pub sample_radius: f64,

    /**
        A number getween 0 and 1 representing the contribution of a single supersample ray.
        Equal to 1/options.supersampling
    */
    pub supersample_power: f64,

    /**
    Additional scene rendering options.
    Scene content, lights, sample rates, etc. are stored here
    */
    pub options: Options,

    /**
        Used to generate a random angle between 0 and 2π
    */
    angle_distribution: Uniform<f64>,
}

/**
User-configuratble description of the scene to render.
Used by Scene to compute additional properties
*/
pub struct Options {
    // width/height of generated image
    pub dimensions: (u16, u16),

    // The primitives in the scene
    pub content: Box<Primitive>,

    pub lights: Vec<Box<Light>>, // point-light sources in the scene
    pub ambient: Color, // ambient lighting

    pub eye: Point,
    pub view: Vector,
    pub up: Vector,

    pub fov: f64, // field of view

    // Represents the number of supersample rays to emit per pixel
    pub supersampling: u8,
}

impl Scene {
    pub fn new(options: Options) -> Scene {
        // The Auxilary Vector is normal to the view and up vectors
        let (_, height) = options.dimensions;

        let aux = options.view.cross(&options.up);
        let up = aux.cross(&options.view).normalize();
        let aux = aux.normalize();

        // First point of the target plane will be at this distance from the eye
        let distance = len(&options.view);

        // Half the height of the point grid in model coordinates
        let ymax = distance * f64::tan((1.0/360.0) * options.fov * f64::consts::PI);

        let sample_radius = ymax / (height as f64);
        let supersample_power = 1.0/(options.supersampling as f64).max(1.0);

        Scene {
            eye: options.eye,
            view: options.view,
            up, aux, sample_radius, supersample_power,
            options,
            angle_distribution: Uniform::new(0.0, f64::consts::PI)
        }
    }

    // Find the intersection for the ray defined by the given eye/direction If no intersection is
    // found, returns an intersetion at infinity with the root content primitive.
    pub fn intersect(&self, e: &Point, d: &Direction) -> (Intersection, &Primitive) {
        self.options.content.intersect(e, d)
    }

    /**
    Get a random angle between 0 and π
    */
    pub fn random_angle(&self, rng: &mut impl Rng) -> f64 {
        self.angle_distribution.sample(rng)
    }

}
