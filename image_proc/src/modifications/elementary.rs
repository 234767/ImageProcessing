//Elementary operations on images enabling to easily modify brightness and contrast
use super::Transformation;
use image::{Pixel, RgbImage};
use num;

//(B3) Negative (--negative)
pub struct Negative {}

impl Transformation for Negative {
    fn apply(&self, image: &mut RgbImage) {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|x| 255 - x);
        }
    }
}

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
