use space::Color;
use std::ops::{Index, IndexMut};

/// RGBA pixel representation, with A being the Alpha channel
/// Each item has a color value between 0 and 255
pub type Pixel = [u8; 4];

/// Lineraly stored container of pixels
/// Index access assumes that memory is arranged in row-major order
pub trait PixelBuffer: Index<usize> + IndexMut<usize> {
    fn raw_pixels(&self) -> *const Pixel;
    fn raw_pixels_mut(&mut self) -> *mut Pixel;
    fn save(&self, _filename: &str) { unimplemented!() }
}

impl PixelBuffer for Vec<Pixel> {
    fn raw_pixels(&self) -> *const Pixel { self[..].as_ptr() }
    fn raw_pixels_mut(&mut self) -> *mut Pixel { self[..].as_mut_ptr() }
}

impl PixelBuffer for [Pixel] {
    fn raw_pixels(&self) -> *const Pixel { self.as_ptr() }
    fn raw_pixels_mut(&mut self) -> *mut Pixel { self.as_mut_ptr() }
}

/// Queriable store of pixels.
/// Returns value of the render function for use in custom image-loading clients.
pub struct Film {
    pub width: u16,
    pub height: u16,
    pub data: Box<PixelBuffer<Output = Pixel>>
}

impl Film {
    pub fn new(width: u16, height: u16) -> Film {
        let data = vec![[0, 0, 0, 0]; (width as usize) * (height as usize)];
        let data = Box::new(data);
        Film { width, height, data }
    }

    /// Create a new film with a pre-allocated box of data
    /// Use this to avoid using extra memory when writing to
    pub fn new_with_data(width: u16, height: u16, data: Box<PixelBuffer<Output = Pixel>>) -> Film {
        Film { width, height, data }
    }

    pub fn get(&self, x: u16, y: u16) -> &Pixel {
        let offset = self.offset(x, y);
        &self.data[offset]
    }

    pub fn set(&mut self, x: u16, y: u16, color: &Color) {
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
    fn offset(&self, x: u16, y: u16) -> usize {
        (self.width as usize) * (y as usize) + (x as usize)
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
