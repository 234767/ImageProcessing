use crate::modifications::Transformation;
use image::{Pixel, RgbImage};

//(B3) Negative (--negative)
pub struct Negative;

impl Transformation for Negative {
    fn apply(&self, image: &mut RgbImage) {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|x| 255 - x);
        }
    }
}
