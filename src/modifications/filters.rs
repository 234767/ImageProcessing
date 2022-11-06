//Methods of image noise removal
use crate::modifications::Transformation;
use crate::parsing::Args;
use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;

use num::Integer;

fn is_in_range(x: u32, y: u32, image: &RgbImage) -> bool {
    x < image.width() && y < image.height()
}

//(N1) Median filter (--median)
pub struct MedianFilter {
    width: u32,
    height: u32,
}

impl MedianFilter {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let mut width: u32 = args.try_get_arg("-w")?;
        if width.is_even() {
            width += 1
        }
        let mut height: u32 = args.try_get_arg("-h")?;
        if height.is_even() {
            height += 1
        }
        Ok(Self { width, height })
    }
}

impl Transformation for MedianFilter {
    fn apply(&self, image: &mut RgbImage) {
        let width_offset = self.width / 2;
        let height_offset = self.height / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let mut old_pixels: Vec<&Rgb<u8>> = Vec::new();
            for x in u32::saturating_sub(target_x, width_offset)..(target_x + width_offset) {
                for y in u32::saturating_sub(target_y, height_offset)..(target_y + height_offset) {
                    if is_in_range(x, y, image) {
                        old_pixels.push(image.get_pixel(x, y));
                    }
                }
            }
            for channel in 0..3 {
                let mut luminosities: Vec<u8> =
                    old_pixels.iter().map(|pixel| pixel[channel]).collect();
                luminosities.sort();
                new_pixel[channel] = luminosities[luminosities.len() / 2];
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
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let mut width: u32 = args.try_get_arg("-w")?;
        if width.is_even() {
            width += 1
        }
        let mut height: u32 = args.try_get_arg("-h")?;
        if height.is_even() {
            height += 1
        }
        Ok(Self { width, height })
    }
}

impl Transformation for GeometricMeanFilter {
    fn apply(&self, image: &mut RgbImage) {
        let mut h_offset = self.height / 2;
        let mut w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let mut old_pixels: Vec<&Rgb<u8>> = Vec::new();
            for x in u32::saturating_sub(target_x, w_offset)..(target_x + w_offset) {
                for y in u32::saturating_sub(target_y, h_offset)..(target_y + h_offset) {
                    if is_in_range(x, y, image) {
                        old_pixels.push(image.get_pixel(x, y));
                    }
                }
            }
            for channel in 0..3 {
                let mut luminosities: Vec<u8> =
                    old_pixels.iter().map(|pixel| pixel[channel]).collect();
                new_pixel[channel] =
                    luminosities.iter().sum::<u8>() / luminosities.iter().count() as u8;
            }
        }
        *image = new_image;
    }
}
