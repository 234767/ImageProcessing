use crate::modifications::is_edge;
use crate::modifications::Transformation;
use image::{ImageBuffer, RgbImage};
use num::traits::Pow;

pub struct SobelOperator;

impl Transformation for SobelOperator {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            if is_edge(image, x, y) {
                continue;
            }
            for channel in 0..3 {
                let sobel_x = image.get_pixel(x + 1, y - 1)[channel] as f64
                    + 2.0 * image.get_pixel(x + 1, y)[channel] as f64
                    + image.get_pixel(x + 1, y + 1)[channel] as f64
                    - (image.get_pixel(x - 1, y - 1)[channel] as f64
                        + 2.0 * image.get_pixel(x - 1, y)[channel] as f64
                        + image.get_pixel(x - 1, y + 1)[channel] as f64);
                let sobel_y = image.get_pixel(x - 1, y - 1)[channel] as f64
                    + 2.0 * image.get_pixel(x, y - 1)[channel] as f64
                    + image.get_pixel(x + 1, y - 1)[channel] as f64
                    - (image.get_pixel(x - 1, y + 1)[channel] as f64
                        + 2.0 * image.get_pixel(x, y + 1)[channel] as f64
                        + image.get_pixel(x + 1, y + 1)[channel] as f64);
                pixel[channel] =
                    f64::sqrt(f64::pow(sobel_x as f64, 2.0) + f64::pow(sobel_y as f64, 2.0)) as u8;
            }
        }
        *image = new_image;
    }
}
