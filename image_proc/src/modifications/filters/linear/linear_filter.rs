use crate::modifications::is_edge;
use crate::modifications::Transformation;
use image::RgbImage;

pub struct LinearFilter {
    mask: [[f64; 3]; 3],
    mask_scale: f64,
}

impl LinearFilter {
    pub fn new(mask: [[f64; 3]; 3], mask_scale: Option<f64>) -> Self {
        Self {
            mask,
            mask_scale: mask_scale.unwrap_or(1.0),
        }
    }
    pub fn from_flat_mask(flat_mask: [f64; 9], mask_scale: Option<f64>) -> Self {
        let mut mask = [[0f64; 3]; 3];
        for y in 0..3 {
            for x in 0..3 {
                mask[y][x] = flat_mask[3 * y + x]
            }
        }
        Self {
            mask,
            mask_scale: mask_scale.unwrap_or(1.0),
        }
    }
}

impl Transformation for LinearFilter {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = RgbImage::new(image.width(), image.height());
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            if is_edge(image, x, y) {
                *pixel = *image.get_pixel(x, y);
                continue;
            }

            for channel in 0..3 {
                let mut sum = 0f64;
                for i in 0..3 {
                    for j in 0..3 {
                        sum += (self.mask[j as usize][i as usize])
                            * image.get_pixel(x + i - 1, y + j - 1)[channel] as f64;
                    }
                }
                pixel[channel] = (sum * self.mask_scale) as u8;
            }
        }
        *image = new_image;
    }
}
