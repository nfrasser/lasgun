use space::Color;

/// Each item has a color value between 0 and 255
pub type Pixel = [u8; 3];

/// An image is anything that can set its own pixel colour
/// The parameter represents the size of the max dimension paramter
/// e.g., D == u16 means the image can have a max resolution of 6
pub trait Film {
    fn set_pixel_color(&mut self, x: u16, y: u16, color: &Color);
}

/// Queriable store of pixels.
/// Returns value of the render function for use in custom image-loading clients.
pub struct ImageBuffer {
    pub width: u16,
    pub height: u16,
    pixels: Vec<Pixel>
}

impl ImageBuffer {
    pub fn new(width: u16, height: u16) -> ImageBuffer {
        let size = width as usize * height as usize;
        ImageBuffer {
            width, height,
            pixels: vec!([0, 0, 0]; size)
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &Pixel {
        let offset = self.offset(x, y);
        &self.pixels[offset]
    }

    pub fn foreach<F>(&self, func: F) where F: Fn(&Pixel, u16, u16) -> () {
        let mut offset = 0;
        for x in 0..self.width {
            for y in 0..self.height {
                func(&self.pixels[offset], x, y);
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

impl Film for ImageBuffer {
    fn set_pixel_color(&mut self, x: u16, y: u16, color: &Color) {
        let offset = self.offset(x, y);
        self.pixels[offset] = [
            to_byte(color.x),
            to_byte(color.y),
            to_byte(color.z),
        ]
    }
}

/**
Convert a colour channel from between 0 and 1 to an interger between 0 and 255
*/
#[inline]
fn to_byte(channel: f64) -> u8 {
    (channel.max(0.0).min(1.0) * 255.0).round() as u8
}
