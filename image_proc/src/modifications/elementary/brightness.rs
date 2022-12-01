use crate::modifications::Transformation;
use image::{Pixel, RgbImage};

//(B1) Image brightness modification (--brightness)
pub struct Brightness {
    amount: i32,
}

impl Brightness {
    pub fn new(amount: i32) -> Self {
        Self { amount }
    }
}

impl Transformation for Brightness {
    fn apply(&self, image: &mut RgbImage) {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|c| num::clamp(c as i32 + self.amount, 0, u8::MAX as i32) as u8);
        }
    }
}
