use crate::modifications::filters::iterating::Neighbourhood;
use crate::modifications::Transformation;
use image::{ImageBuffer, Rgb, RgbImage};

//(N1) Median filter (--median)
pub struct MedianFilter {
    width: u32,
    height: u32,
}

impl MedianFilter {
    impl_new!();
}

impl Transformation for MedianFilter {
    fn apply(&self, image: &mut RgbImage) {
        let width_offset = self.width / 2;
        let height_offset = self.height / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let mut luminosity_buckets = [[0u32; 256]; 3];
            let neighbourhood =
                Neighbourhood::new(image, target_x, width_offset, target_y, height_offset);
            for Rgb(pixel) in neighbourhood.iter() {
                for channel in 0..3 {
                    let luminosity = pixel[channel];
                    luminosity_buckets[channel][luminosity as usize] += 1;
                }
            }
            for channel in 0..3 {
                let luminosity_buckets = luminosity_buckets[channel];
                let median_index = neighbourhood.non_enumerated_count() as u32 / 2;
                let median: u8 = {
                    let mut median: u8 = 255;
                    let mut partial_sum: u32 = 0;
                    for l in 0..=255 {
                        partial_sum += luminosity_buckets[l];
                        if partial_sum > median_index {
                            median = l as u8;
                            break;
                        }
                    }
                    median
                };
                new_pixel[channel] = median;
            }
        }
        *image = new_image;
    }
}
