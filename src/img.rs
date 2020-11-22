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

/// Queriable store of pixels that will eventually be saved to a file. By
/// default, pixel data is internally represented by a Vector of pixels arranged
/// in row-major order.
pub struct Film {
    pub w: u32,
    pub h: u32,
    pub winv: f64,
    pub hinv: f64,
    pub aspect: f64,

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
        Film {
            w: width,
            h: height,
            winv: 1. / width as f64,
            hinv: 1. / height as f64,
            aspect: width as f64 / height as f64,
            output
        }
    }
}

impl Index<usize> for Film {
    type Output = Pixel;
    #[inline] fn index(&self, at: usize) -> &Self::Output { &self.output[at] }
}

impl IndexMut<usize> for Film {
    #[inline] fn index_mut(&mut self, at: usize) -> &mut Self::Output { &mut self.output[at] }
}

impl PixelBuffer for Film {
    #[inline] fn save(&self, filename: &str) { self.output.save(filename) }
}

impl Img for Film {
    #[inline] fn w(&self) -> u32{ self.w }
    #[inline] fn h(&self) -> u32 { self.h }
    #[inline] fn winv(&self) -> f64 { self.winv }
    #[inline] fn hinv(&self) -> f64 { self.hinv }
    #[inline] fn aspect(&self) -> f64 { self.aspect }
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
