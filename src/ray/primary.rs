use space::{ Vector, Point, Color };
use scene::Scene;
use ray::Ray;

pub struct PrimaryRay {
    eye: Point, // eye position
    direction: Vector // direction from eye into which the ray is cast
}

impl PrimaryRay {
    pub fn new(eye: Point, direction: Vector) -> PrimaryRay {
        PrimaryRay { eye, direction  }
    }
}

impl Ray for PrimaryRay {
    fn cast(&self, scene: &Scene) -> Color {
        let (intersection, primitive) = scene.intersect(&self.eye, &self.direction);
        let material = primitive.material();
        let direction: Vector = intersection.t * self.direction;
        let point = self.eye + direction;
        material.color(&point, &self.eye, &intersection.normal, scene)
    }
}
