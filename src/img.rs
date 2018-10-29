use crate::{space::Color, Scene};
use std::ops::{Index, IndexMut};

/// RGBA pixel representation, with A being the Alpha channel
/// Each item has a color value between 0 and 255
pub type Pixel = [u8; 4];

/// Linearly-stored container of pixels
/// Index access assumes that memory is arranged in row-major order
pub trait PixelBuffer: Index<usize> + IndexMut<usize> {
    fn raw_pixels(&self) -> *const Pixel;
    fn raw_pixels_mut(&mut self) -> *mut Pixel;
    fn as_slice(&self) -> &[u8];
    fn save(&self, _filename: &str) { unimplemented!() }
}

impl PixelBuffer for Vec<Pixel> {
    fn raw_pixels(&self) -> *const Pixel { self[..].as_ptr() }
    fn raw_pixels_mut(&mut self) -> *mut Pixel { self[..].as_mut_ptr() }
    fn as_slice(&self) -> &[u8] { unsafe { std::mem::transmute(self.as_slice()) } }
}

impl PixelBuffer for [Pixel] {
    fn raw_pixels(&self) -> *const Pixel { self.as_ptr() }
    fn raw_pixels_mut(&mut self) -> *mut Pixel { self.as_mut_ptr() }
    fn as_slice(&self) -> &[u8] { unsafe { std::mem::transmute(self) } }
}

/// Queriable store of pixels that will eventually be saved to a file. By
/// default, pixel data is internally represented by a Vector of pixels arranged
/// in row-major order.
pub struct Film {
    pub width: u16,
    pub height: u16,
    data: Box<dyn PixelBuffer<Output = Pixel> + Send>
}

impl Film {

    /// Initialize a new film with the given dimensions, with each pixel
    /// initialized to Black
    pub fn new(width: u16, height: u16) -> Film {
        let data = vec![[0, 0, 0, 0]; (width as usize) * (height as usize)];
        let data = Box::new(data);
        Film { width, height, data }
    }

    /// Create a new film based on the parameters in the given scene
    pub fn from(scene: &Scene) -> Film {
        Film::new(scene.options.width, scene.options.height)
    }

    /// Create a new film with a pre-allocated box of data/ Use this when a
    /// buffer for an image has already been allocated externally and you want
    /// to avoid using extra memory for caching pixel data.
    ///
    /// Can use this with a Vec or slice of pixels.
    ///
    /// Assumes that that data has room for width * height * 4 bytes worth of
    /// pixels.
    pub fn new_with_data(width: u16, height: u16, data: Box<dyn PixelBuffer<Output = Pixel> + Send>) -> Film {
        Film { width, height, data }
    }

    /// Returns the total number of pixels of the image (width * height)
    pub fn num_pixels(&self) -> usize {
        self.width as usize * self.height as usize
    }

    /// Get the pixel at the given x/y dimensions
    pub fn get(&self, x: usize, y: usize) -> &Pixel {
        let offset = self.offset(x, y);
        &self.data[offset]
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        let offset = self.offset(x, y);
        set_pixel_color(&mut self.data[offset], color)
    }

    pub fn foreach<F>(&self, func: F) where F: Fn(u16, u16, &Pixel) -> () {
        let mut offset = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                func(x, y, &self.data[offset]);
                offset += 1;
            }
        }
    }

    // Retrieves the offset into the pixel vector
    #[inline]
    fn offset(&self, x: usize, y: usize) -> usize {
        (self.width as usize) * y + x
    }
}

impl Index<usize> for Film {
    type Output = Pixel;

    fn index(&self, at: usize) -> &Self::Output {
        &self.data[at]
    }
}

impl IndexMut<usize> for Film {
    fn index_mut(&mut self, at: usize) -> &mut Self::Output {
        &mut self.data[at]
    }
}

impl PixelBuffer for Film {
    fn raw_pixels(&self) -> *const Pixel {
        self.data.raw_pixels()
    }

    fn raw_pixels_mut(&mut self) -> *mut Pixel {
        self.data.raw_pixels_mut()
    }

    fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }

    fn save(&self, filename: &str) {
        self.data.save(filename)
    }
}

/// Set the color of the given pixel
#[inline]
pub fn set_pixel_color(pixel: &mut Pixel, color: &Color) {
    pixel[0] = to_byte(color[0]);
    pixel[1] = to_byte(color[1]);
    pixel[2] = to_byte(color[2]);
    pixel[3] = 255;
}

/// Convert a colour channel from between 0 and 1 to an interger between 0 and 255
#[inline]
fn to_byte(channel: f64) -> u8 {
    (channel.max(0.0).min(1.0) * 255.0).round() as u8
}
