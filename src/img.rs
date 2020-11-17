use crate::space::*;
use std::ops::{Index, IndexMut};

/// RGBA pixel representation, with A being the Alpha channel
/// Each item has a color value between 0 and 255
pub type Pixel = [u8; 4];

/// Linearly-stored container of pixels
/// Index access assumes that memory is arranged in row-major order
pub trait PixelBuffer: Index<usize> + IndexMut<usize> {
    fn raw_pixels(&self) -> *const Pixel;
    fn raw_pixels_mut(&mut self) -> *mut Pixel;
    fn as_bytes(&self) -> &[u8];
    fn save(&self, _filename: &str) { unimplemented!() }
}

impl PixelBuffer for Vec<Pixel> {
    fn raw_pixels(&self) -> *const Pixel { self[..].as_ptr() }
    fn raw_pixels_mut(&mut self) -> *mut Pixel { self[..].as_mut_ptr() }
    fn as_bytes(&self) -> &[u8] { unsafe { std::mem::transmute(self.as_slice()) } }
}

impl PixelBuffer for [Pixel] {
    fn raw_pixels(&self) -> *const Pixel { self.as_ptr() }
    fn raw_pixels_mut(&mut self) -> *mut Pixel { self.as_mut_ptr() }
    fn as_bytes(&self) -> &[u8] { unsafe { std::mem::transmute(self) } }
}

/// Queriable store of pixels that will eventually be saved to a file. By
/// default, pixel data is internally represented by a Vector of pixels arranged
/// in row-major order.
pub struct Film {
    pub resolution: Vector2u,
    pub inv_resolution: Vector2f,
    pub aspect_ratio: f64,

    /// Output pixel buffer that eventually gets written out to disk or wherever
    output: Box<dyn PixelBuffer<Output = Pixel>>,
}

unsafe impl std::marker::Send for Film {}
unsafe impl std::marker::Sync for Film {}

impl Film {

    /// Initialize a new film with the given dimensions, with each pixel
    /// initialized to Black
    pub fn new(width: u32, height: u32) -> Film {
        let area = (width as usize) * (height as usize);
        let output = Box::new(vec![[0, 0, 0, 0]; area]);
        Film::new_with_output(width, height, output)
    }

    /// Create a new film with a pre-allocated box of data/ Use this when a
    /// buffer for an image has already been allocated externally and you want
    /// to avoid using extra memory for caching pixel data.
    ///
    /// Can use this with a Vec or slice of pixels.
    ///
    /// Assumes that that data has room for width * height * 4 bytes worth of
    /// pixels.
    pub fn new_with_output(width: u32, height: u32, output: Box<dyn PixelBuffer<Output = Pixel>>) -> Film {
        let resolution = Vector2u::new(width, height);
        let inv_resolution = Vector2f::new(1. / width as f64, 1. / height as f64);
        let aspect_ratio = width as f64 / height as f64;
        Film { resolution, inv_resolution, aspect_ratio, output }
    }

    /// Returns the total number of pixels of the image (width * height)
    pub fn num_pixels(&self) -> usize {
        self.resolution.x as usize * self.resolution.y as usize
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        let offset = self.offset(x, y);
        set_pixel_color(&mut self.output[offset], color)
    }

    // Retrieves the offset into the pixel vector
    #[inline]
    fn offset(&self, x: usize, y: usize) -> usize {
        (self.resolution.x as usize) * y + x
    }
}

impl Index<usize> for Film {
    type Output = Pixel;

    fn index(&self, at: usize) -> &Self::Output {
        &self.output[at]
    }
}

impl IndexMut<usize> for Film {
    fn index_mut(&mut self, at: usize) -> &mut Self::Output {
        &mut self.output[at]
    }
}

impl PixelBuffer for Film {
    fn raw_pixels(&self) -> *const Pixel {
        self.output.raw_pixels()
    }

    fn raw_pixels_mut(&mut self) -> *mut Pixel {
        self.output.raw_pixels_mut()
    }

    fn as_bytes(&self) -> &[u8] {
        self.output.as_bytes()
    }

    fn save(&self, filename: &str) {
        self.output.save(filename)
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
