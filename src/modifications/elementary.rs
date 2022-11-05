//Elementary operations on images enabling to easily modify brightness and contrast
use crate::modifications::Transformation;
use crate::parsing::Args;
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
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let amount: i32 = args.try_get_arg("amount")?;
        if amount < 0 {
            Err(format!("Number {} is not a positive integer!", amount))
        } else {
            Ok(Self { amount })
        }
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
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let factor: f64 = args.try_get_arg("amount")?;
        if factor < 0.0 {
            Err(format!("Number {} is not a positive integer!", factor))
        } else {
            Ok(Self { factor })
        }
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
