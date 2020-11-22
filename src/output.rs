use std::ops::{Index, IndexMut};
use ::image::RgbaImage;
use crate::{capture, Scene, Film, Pixel, PixelBuffer};

pub fn render(scene: &Scene, resolution: [u32; 2], filename: &str) {
    let (width, height) = (resolution[0], resolution[1]);

    // Pre-allocate traced image data
    let rgba = RgbaImage::new(width, height);
    let image = Box::new(Image(rgba));
    let mut film = Film::new_with_output(width, height, image);

    // Capture the image
    capture(&scene, &mut film);

    // Save the film
    film.save(filename)
}

/// Create a film in the correct x/y dimensions for the given scene
pub fn film(resolution: [u32; 2]) -> Film {
    let (width, height) = (resolution[0], resolution[1]);

    // Pre-allocate traced image data
    let rgba = RgbaImage::new(width, height);
    let image = Box::new(Image(rgba));
    Film::new_with_output(width, height, image)
}

struct Image(RgbaImage);

impl Index<usize> for Image {
    type Output = Pixel;
    #[inline]
    fn index(&self, index: usize) -> &Pixel {
        let (x, y) = (
            (index % self.0.width() as usize) as u32,
            (index / self.0.width() as usize) as u32
        );
        &self.0.get_pixel(x, y).0
    }
}

impl IndexMut<usize> for Image {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Pixel {
        let (x, y) = (
            (index % self.0.width() as usize) as u32,
            (index / self.0.width() as usize) as u32
        );
        &mut self.0.get_pixel_mut(x, y).0
    }
}

impl PixelBuffer for Image {
    fn save(&self, filename: &str) { self.0.save(filename).unwrap() }
}
