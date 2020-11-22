use std::ops::{Index, IndexMut};
use crate::img::*;

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
