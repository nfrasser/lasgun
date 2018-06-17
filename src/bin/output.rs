extern crate image;
extern crate lasgun;

use image::{Rgb, RgbImage};
use lasgun::{Scene, Image, Color};

pub fn render(scene: &Scene, filename: &str) {
    let (width, height) = scene.dimensions;
    let buffer = RgbImage::new(width as u32, height as u32);
    let mut image = OutputImage(buffer);
    lasgun::render_to(&mut image, &scene);
    image.0.save(filename).unwrap();
}

struct OutputImage(RgbImage);
impl Image for OutputImage {
    fn set_pixel_color(&mut self, x: u16, y: u16, color: &Color) {
        let pixel = Rgb {
            data: [
                to_byte(color.x),
                to_byte(color.y),
                to_byte(color.z)
            ]
        };
        self.0.put_pixel(x as u32, y as u32, pixel)
    }
}

// Convert a colour channel between 0 and 1 to an interger between 0 and 255
#[inline]
fn to_byte(channel: f64) -> u8 {
    (channel.max(0.0).min(1.0) * 255.0).round() as u8
}
