use crate::modifications::{
    frequency_domain::fourier_transform::{dft_2d, FFTDirection},
    Transformation,
};
use image::{DynamicImage, GrayImage, Luma, Rgb, RgbImage};
use num::{complex::ComplexFloat, Complex};
use std::convert::identity;

fn to_grayscale(image: &RgbImage) -> GrayImage {
    image::imageops::grayscale(image)
}

fn to_rgb(image: GrayImage) -> RgbImage {
    DynamicImage::from(image).to_rgb8()
}

fn normalize(value: f64, max_value: f64) -> Luma<u8> {
    let normalization_factor = u8::MAX as f64 / f64::ln(1.0 + max_value);
    let value = (normalization_factor * f64::ln(1.0 + value)).clamp(0.0, u8::MAX as f64);
    Luma([value as u8])
}

fn assert_pow_2(num: &u32) {
    let log = (*num as f64).log2();
    assert!(log - log.floor() < 1e-5, "Number must be a power of 2");
}

fn get_swapped_coordinates(x: u32, y: u32, width: u32, height: u32) -> (u32, u32) {
    use core::cmp::Ordering::*;
    match (x.cmp(&(width / 2)), y.cmp(&(height / 2))) {
        (Less, Less) => (x + width / 2, y + height / 2),
        (Less, _) => (x + width / 2, y - height / 2),
        (_, Less) => (x - width / 2, y + height / 2),
        (_, _) => (x - width / 2, y - height / 2),
    }
}

pub struct DFT;

impl DFT {
    fn apply(image: &RgbImage) -> Vec<Vec<Complex<f64>>> {
        assert_pow_2(&image.height());
        assert_pow_2(&image.width());

        let pixels: Vec<Vec<_>> = to_grayscale(image)
            .rows()
            .into_iter()
            .map(|row| row.map(|Luma([x])| *x as f64 / u8::MAX as f64).collect())
            .collect();

        let pixels_as_slice: Vec<_> = pixels.iter().map(|x| x.as_slice()).collect();

        dft_2d(pixels_as_slice.as_slice(), FFTDirection::Forward)
    }
}

impl Transformation for DFT {
    fn apply(&self, image: &mut RgbImage) {
        let transformed = Self::apply(image);

        let max_value = {
            let mut max = 0.0;
            for value in transformed.iter().flat_map(identity) {
                if value.abs() > max {
                    max = value.abs();
                }
            }
            max
        };

        let magnitude = GrayImage::from_fn(image.width(), image.height(), |x, y| {
            let (x,y) = get_swapped_coordinates(x,y,image.width(), image.height());
            normalize(transformed[y as usize][x as usize].abs(), max_value)
        });

        *image = to_rgb(magnitude);
    }
}
