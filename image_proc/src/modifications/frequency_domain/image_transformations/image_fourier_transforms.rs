use super::util::*;
use crate::modifications::frequency_domain::fourier_transform::{dft_2d, FFTDirection};
use crate::modifications::Transformation;
use image::{GrayImage, Luma, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;
use std::convert::identity;

pub trait ImageFourierTransform {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>>;
    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>>;
}

pub struct DFT;

impl ImageFourierTransform for DFT {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>> {
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

    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        let data_as_slice: Vec<_> = data.iter().map(|x| x.as_slice()).collect();
        dft_2d(data_as_slice.as_slice(), FFTDirection::Forward)
    }
}

impl Transformation for DFT {
    fn apply(&self, image: &mut RgbImage) {
        let transformed = Self::transform(image);

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
            let (x, y) = swap_quadrant_coordinates(x, y, image.width(), image.height());
            normalize(transformed[y as usize][x as usize].abs(), max_value)
        });

        *image = to_rgb(magnitude);
    }
}
