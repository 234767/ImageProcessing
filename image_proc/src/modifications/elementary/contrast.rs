use image::{Pixel, RgbImage};
use crate::modifications::Transformation;

//(B2) Image contrast modification (--contrast)
pub struct Contrast {
    factor: f64,
}

impl Contrast {
    pub fn new(factor: f64) -> Self {
        Self { factor }
    }
}

impl Transformation for Contrast {
    fn apply(&self, image: &mut RgbImage) {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|c| {
                num::clamp(
                    (c as f64 - 128f64) * self.factor + 128f64,
                    0f64,
                    u8::MAX as f64,
                ) as u8
            });
        }
    }
}
