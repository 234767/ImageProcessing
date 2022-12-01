use crate::modifications::filters::is_edge;
use crate::modifications::Transformation;
use image::{ImageBuffer, Rgb, RgbImage};
use num::pow::Pow;

pub struct UolisOperator;

impl Transformation for UolisOperator {
    fn apply(&self, image: &mut RgbImage) {
        const NORMALIZATION_FACTOR: f64 = 2550.0;

        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            if is_edge(image, x, y) {
                continue;
            }
            let neighbors = {
                let mut neighbors: Vec<&Rgb<u8>> = vec![];
                for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let xi = (x as i32 + i) as u32;
                    let yi = (y as i32 + j) as u32;
                    if xi < image.width() && yi < image.height() {
                        neighbors.push(image.get_pixel(xi, yi));
                    }
                }
                neighbors
            };
            for channel in 0..3 {
                let product = neighbors.iter().map(|x| x[channel] as f64).product::<f64>();
                let power = (image.get_pixel(x, y)[channel] as f64).pow(4.0);
                let log_base = power / product;
                let log = f64::log10(log_base);
                pixel[channel] = (NORMALIZATION_FACTOR * log / 4.0) as u8
            }
        }
        *image = new_image;
    }
}
