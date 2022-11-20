use std::cmp::min;
use std::env::Args;
use super::super::Transformation;
use super::iterating::Neighbourhood;
use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;
use num::pow::Pow;

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

//(N1) geometric mean filter (--gmean)
pub struct GeometricMeanFilter {
    width: u32,
    height: u32,
}

impl GeometricMeanFilter {
    impl_new!();
}

impl Transformation for GeometricMeanFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let neighbourhood = Neighbourhood::new(image, target_x, w_offset, target_y, h_offset);
            let products = neighbourhood
                .iter()
                .fold([1.0, 1.0, 1.0], |prod, Rgb(pixel)| {
                    [
                        prod[0] * pixel[0] as f64,
                        prod[1] * pixel[1] as f64,
                        prod[2] * pixel[2] as f64,
                    ]
                });
            for channel in 0..3 {
                new_pixel[channel] = f64::pow(
                    products[channel],
                    1f64 / neighbourhood.non_enumerated_count() as f64,
                ) as u8;
            }
        }
        *image = new_image;
    }
}

pub struct MaxFilter {
    width: u32,
    height: u32,
}

impl MaxFilter {
    impl_new!();
}

impl Transformation for MaxFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, Rgb(new_pixel)) in new_image.enumerate_pixels_mut() {
            let neighbourhood = Neighbourhood::new(image, target_x, w_offset, target_y, h_offset);
            let max_values = neighbourhood.iter().fold([0u8, 0, 0], |prod, Rgb(pixel)| {
                [
                    u8::max(prod[0], pixel[0]),
                    u8::max(prod[1], pixel[1]),
                    u8::max(prod[2], pixel[2]),
                ]
            });
            *new_pixel = max_values
        }
        *image = new_image;
    }
}

pub struct MinimumFilter {
    width: u32,
    height: u32,
}

impl MinimumFilter {
    impl_new!();
}
//Code for Minimum filter updated to the new task

impl Transformation for MinimumFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let neighbourhood = Neighbourhood::new(image, target_x, w_offset, target_y, h_offset);
            let min_values = neighbourhood.iter().fold([0u8, 0, 0], |prod, Rgb(pixel)| {
                [
                    u8::min(prod[0], pixel[0]),
                    u8::min(prod[1], pixel[1]),
                    u8::min(prod[2], pixel[2]),
                ]
            });
            *new_pixel = Rgb::from(min_values)
        }
        *image = new_image;
    }
}


//Code for Minimum Filter presented while working
/*
impl Transformation for MinimumFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let old_pixels: Vec<&Rgb<u8>> = collect_pixels(image, target_x, w_offset, target_y, h_offset);
            for channel in 0..3 {
                let mut luminosities: Vec<u8> =
                    old_pixels.iter().map(|pixel| pixel[channel]).collect();
                    luminosities.sort();
                    new_pixel[channel] = luminosities.iter().fold(255u8,|a,b| min(a, *b)) as u8;
            }
        }
        *image = new_image;
    }
}
*/