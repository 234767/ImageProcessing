use crate::modifications::is_edge;
use crate::modifications::Transformation;
use image::{ImageBuffer, RgbImage};
use num::traits::Pow;

pub struct RobertsOperator1;

impl Transformation for RobertsOperator1 {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            if is_edge(image, x, y) {
                continue;
            }
            for channel in 0..3 {
                let difference_1: i16 = image.get_pixel(x, y)[channel] as i16
                    - image.get_pixel(x + 1, y + 1)[channel] as i16;
                let difference_2: i16 = image.get_pixel(x, y + 1)[channel] as i16
                    - image.get_pixel(x + 1, y)[channel] as i16;
                pixel[channel] =
                    f64::sqrt((difference_1 as f64).pow(2) + (difference_2 as f64).pow(2))
                        .clamp(0.0, 255.0) as u8;
            }
        }
        *image = new_image;
    }
}
