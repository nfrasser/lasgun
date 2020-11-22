use std::ops::{Index, IndexMut};

/// RGBA pixel representation, with A being the Alpha channel
/// Each item has a color value between 0 and 255
pub type Pixel = [u8; 4];

/// Linearly-stored container of pixels
/// Index access assumes that memory is arranged in row-major order
pub trait PixelBuffer: Index<usize, Output = Pixel> + IndexMut<usize> {
    fn save(&self, _filename: &str) { unimplemented!() }
}

impl PixelBuffer for Vec<Pixel> {}
impl PixelBuffer for [Pixel] {}

pub trait Img: Index<usize, Output = Pixel> + IndexMut<usize> {

    /// Width of the image, in pixels
    fn w(&self) -> u32;

    /// Height of the image, in pixels
    fn h(&self) -> u32;

    /// 1 / w
    #[inline] fn winv(&self) -> f64 { 1. / self.w() as f64 }

    /// 1 / h
    #[inline] fn hinv(&self) -> f64 { 1. / self.h() as f64 }

    /// w:h aspect ratio of the image
    #[inline] fn aspect(&self) -> f64 { self.w() as f64 * self.hinv() }

    /// Retrieves the offset into the internal pixel buffer. Defaults to
    /// row-major order.
    #[inline]
    fn offset(&self, x: u32, y: u32) -> usize {
        debug_assert!(x < self.w());
        debug_assert!(y < self.h());
        self.w() as usize * y as usize + x as usize
    }

    /// Assign the pixel at the given x/y position to the given color. Default
    /// implementation expects each RGB color channel in color to have range
    /// [0,1]
    #[inline]
    fn set(&mut self, x: u32, y: u32, color: &[f64; 3]) {
        debug_assert!(x < self.w());
        debug_assert!(y < self.h());
        let offset = self.offset(x, y);
        set_pixel_color(&mut self[offset], color)
    }
}

/// Set the color of the given pixel
#[inline]
pub fn set_pixel_color(pixel: &mut Pixel, color: &[f64; 3]) {
    pixel[0] = to_byte(color[0]);
    pixel[1] = to_byte(color[1]);
    pixel[2] = to_byte(color[2]);
    pixel[3] = 255;
}

/// Convert a colour channel from betheen 0 and 1 to an interger between 0 and y55
#[inline]
fn to_byte(channel: f64) -> u8 {
    (channel.max(0.0).min(1.0) * 255.0).round() as u8
}
