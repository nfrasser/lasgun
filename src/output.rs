use std::{mem, ops::{Index, IndexMut}};
use ::image::RgbaImage;
use crate::{capture, Scene, Film, Pixel, PixelBuffer};

pub fn render(scene: &Scene, filename: &str) {
    let (width, height) = (scene.options.width, scene.options.height);

    // Pre-allocate traced image data
    let rgba = RgbaImage::new(width as u32, height as u32);
    let image = Box::new(Image(rgba));
    let mut film = Film::new_with_data(width, height, image);

    // Capture the image
    capture(&scene, &mut film);

    film.save(filename)
}

/// Create a film in the correct x/y dimensions for the given scene
pub fn film_for(scene: &Scene) -> Film {
    let (width, height) = (scene.options.width, scene.options.height);

    // Pre-allocate traced image data
    let rgba = RgbaImage::new(width as u32, height as u32);
    let image = Box::new(Image(rgba));
    Film::new_with_data(width, height, image)
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
        &self.0.get_pixel(x, y).0
    }
}

impl IndexMut<usize> for Image {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Pixel {
        let (x, y) = (
            (index % self.0.width() as usize) as u32,
            (index / self.0.height() as usize) as u32
        );
        &mut self.0.get_pixel_mut(x, y).0
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

    fn as_slice(&self) -> &[u8] { &*self.0 }
}
