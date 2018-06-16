use space::{ Vector, Point, Color, Direction };
use scene::Scene;
use ray::Ray;

pub struct PrimaryRay {
    pub e: Point, // eye/camera position in space
    pub d: Direction, // direction from eye into which the ray is cast
}

impl PrimaryRay {
    pub fn new(e: Point, d: Vector) -> PrimaryRay {
        PrimaryRay { e, d: Direction::new(d) }
    }
}

impl Ray for PrimaryRay {
    fn cast(&self, scene: &Scene) -> Color {
        let (intersection, primitive) = scene.intersect(&self.e, &self.d);
        let material = primitive.material();
        let direction: Vector = intersection.t * self.d.vec;
        let point = self.e + direction;
        material.color(&point, &self.e, &intersection.normal, scene)
    }
}
