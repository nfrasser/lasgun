extern crate image;
extern crate lasgun;

use image::{RgbImage};
use lasgun::{Scene, Film, Pixel};

pub fn render(scene: &Scene, filename: &str) {
    let (width, height) = scene.options.dimensions;
    let buffer = RgbImage::new(width as u32, height as u32);
    let mut image = OutputImage(buffer);
    lasgun::capture(&scene, &mut image);
    image.0.save(filename).unwrap();
}

struct OutputImage(RgbImage);
impl Film for OutputImage {
    fn pixel(&self, x: u16, y: u16) -> &Pixel {
        &self.0.get_pixel(x as u32, y as u32).data
    }

    fn pixel_mut(&mut self, x: u16, y: u16) -> &mut Pixel {
        &mut self.0.get_pixel_mut(x as u32, y as u32).data
    }
}
