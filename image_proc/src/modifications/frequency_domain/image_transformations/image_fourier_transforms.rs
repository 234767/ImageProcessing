use super::super::fourier_transform::{dft_2d, fft_2d, FTDirection};
use super::util::*;
use crate::modifications::Transformation;
use image::{GrayImage, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;

pub trait ImageFourierTransform {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>>;
    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>>;
}

pub struct DFT;

impl ImageFourierTransform for DFT {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>> {
        assert_pow_2(image.height());
        assert_pow_2(image.width());

        let pixels = image_to_matrix(image);

        dft_2d(&pixels, FTDirection::Forward)
    }

    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        assert_pow_2(data.len() as u32);
        assert_pow_2(data[0].len() as u32);
        dft_2d(data, FTDirection::Forward)
    }
}

impl Transformation for DFT {
    fn apply(&self, image: &mut RgbImage) {
        let transformed = Self::transform(image);

        let max_value = max(&transformed, |x| x.abs());

        let magnitude = GrayImage::from_fn(image.width(), image.height(), |x, y| {
            let (x, y) = swap_quadrant_coordinates(x, y, image.width(), image.height());
            normalize(transformed[y as usize][x as usize].abs(), max_value)
        });

        *image = to_rgb(magnitude);
    }
}

pub struct FFT;

impl ImageFourierTransform for FFT {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>> {
        assert_pow_2(image.height());
        assert_pow_2(image.width());

        let pixels = image_to_matrix(image);

        fft_2d(&pixels, FTDirection::Forward)
    }

    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        assert_pow_2(data.len() as u32);
        assert_pow_2(data[0].len() as u32);
        fft_2d(data, FTDirection::Inverse)
    }
}

impl Transformation for FFT {
    fn apply(&self, image: &mut RgbImage) {
        let transformed = Self::transform(image);

        let max_value = max(&transformed, |x| x.abs());

        let magnitude = GrayImage::from_fn(image.width(), image.height(), |x, y| {
            let (x, y) = swap_quadrant_coordinates(x, y, image.width(), image.height());
            normalize(transformed[y as usize][x as usize].abs(), max_value)
        });

        *image = to_rgb(magnitude);
    }
}
