extern crate image;
extern crate lasgun;

use std::{mem, ops::{Index, IndexMut}};
use image::RgbaImage;
use lasgun::{Scene, Film, Pixel, PixelBuffer};

pub fn render(scene: &Scene, filename: &str) {
    let (width, height) = scene.options.dimensions;

    // Pre-allocate traced image data
    let rgba = RgbaImage::new(width as u32, height as u32);
    let image = Box::new(Image(rgba));
    let mut film = Film::new_with_data(width, height, image);

    // Capture the image
    lasgun::capture(&scene, &mut film);
    film.data.save(filename)
}

struct Image(RgbaImage);

impl Index<usize> for Image {
    type Output = Pixel;
    #[inline]
    fn index(&self, index: usize) -> &Pixel {
        let (x, y) = (
            (index % self.0.width() as usize) as u32,
            (index / self.0.height() as usize) as u32
        );
        &self.0.get_pixel(x, y).data
    }
}

impl IndexMut<usize> for Image {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Pixel {
        let (x, y) = (
            (index % self.0.width() as usize) as u32,
            (index / self.0.height() as usize) as u32
        );
        &mut self.0.get_pixel_mut(x, y).data
    }
}

impl PixelBuffer for Image {
    fn save(&self, filename: &str) { self.0.save(filename).unwrap() }

    // Get the image container data as lasgun Pixels ([u8; 4])

    fn raw_pixels(&self) -> *const Pixel {
        let pixels: &[Pixel] = unsafe { mem::transmute(&*self.0) };
        pixels.as_ptr()
    }

    fn raw_pixels_mut(&mut self) -> *mut Pixel {
        let pixels: &mut [Pixel] = unsafe { mem::transmute(&mut *self.0) };
        pixels.as_mut_ptr()
    }
}
