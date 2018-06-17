extern crate nalgebra as na;

mod space;
mod math;
mod img;

pub mod material;
pub mod shape;
pub mod primitive;
pub mod light;
pub mod ray;

pub mod scene;

pub use space::{Point, Color, Vector};
pub use scene::Scene;
pub use img::{Image, ImageBuffer};

pub fn render_to<D>(image: &mut Image<D>, scene: &Scene) {

}

pub fn render(scene: &Scene) -> ImageBuffer {
    let (width, height) = scene.dimensions;
    let mut image = ImageBuffer::new(width, height);
    render_to(&mut image, scene);
    image
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
