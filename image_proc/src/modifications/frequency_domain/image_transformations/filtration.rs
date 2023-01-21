use super::{
    image_fourier_transforms::{ImageFourierTransform, FFT},
    util::*,
};
use crate::modifications::geometric::DiagonalFlip;
use crate::modifications::Transformation;
use image::{GrayImage, Luma, RgbImage};
use std::convert::identity;
use num::complex::ComplexFloat;

mod debug_utils {
    use num::Complex;
    use super::*;

    pub fn save_image(
        data: &Vec<Vec<Complex<f64>>>,
        file_name: &str,
        logarithmic_optimization: bool,
    ) {
        let height = data.len() as u32;
        let width = data.first().unwrap().len() as u32;
        let max_value = {
            let mut max = 0.0;
            for value in data.iter().flat_map(identity) {
                if value.abs() > max {
                    max = value.abs();
                }
            }
            max
        };

        let masked = GrayImage::from_fn(width, height, |x, y| {
            let (x, y) = swap_quadrant_coordinates(x, y, width, height);
            let (x, y) = (x as usize, y as usize);
            if logarithmic_optimization {
                normalize(data[y][x].abs(), max_value)
            } else {
                let luma = data[y][x].abs() / max_value * u8::MAX as f64;
                Luma([luma as u8])
            }
        });
        let _ = masked.save(file_name);
    }
}

fn apply_mask_filter<TFourier, TMask>(image: &mut RgbImage, mask: &TMask)
where
    TFourier: ImageFourierTransform,
    TMask: Fn(u32, u32) -> f64,
{
    let mut transform = TFourier::transform(image);
    for (y, row) in transform.iter_mut().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let (x, y) =
                swap_quadrant_coordinates(x as u32, y as u32, image.width(), image.height());
            let mask_value: f64 = mask(x as u32, y as u32);
            *pixel *= mask_value;
        }
    }

    if cfg!(debug_assertions) {
        debug_utils::save_image(&transform, "_mask.debug.bmp", true);
    }

    let inverse = TFourier::inverse(&transform);

    let max_value = {
        let mut max = 0.0;
        for value in inverse.iter().flat_map(identity) {
            if value.abs() > max {
                max = value.abs();
            }
        }
        max
    };

    let result = GrayImage::from_fn(image.width(), image.height(), |x, y| {
        let (x, y) = (x as usize, y as usize);
        let luma = inverse[y][x].abs() / max_value * u8::MAX as f64;
        Luma([luma as u8])
    });

    *image = to_rgb(result);
    DiagonalFlip {}.apply(image)
}

pub struct LowPassFilter {
    radius: u32,
}

impl LowPassFilter {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}

impl Transformation for LowPassFilter {
    fn apply(&self, image: &mut RgbImage) {
        let radius_squared = self.radius * self.radius;
        let half_width = image.width() / 2;
        let half_height = image.height() / 2;
        let mask = move |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            if x * x + y * y <= radius_squared {
                1.0
            } else {
                0.0
            }
        };
        apply_mask_filter::<FFT, _>(image, &mask);
    }
}

pub struct HighPassFilter {
    radius: u32,
}

impl HighPassFilter {
    pub fn new(radius: u32) -> Self {
        Self { radius }
    }
}

impl Transformation for HighPassFilter {
    fn apply(&self, image: &mut RgbImage) {
        let radius_squared = self.radius * self.radius;
        let half_width = image.width() / 2;
        let half_height = image.height() / 2;
        let mask = move |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            if x * x + y * y > radius_squared {
                1.0
            } else {
                0.0
            }
        };
        apply_mask_filter::<FFT, _>(image, &mask);
    }
}
