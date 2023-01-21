use super::util::*;
use crate::modifications::frequency_domain::fourier_transform::{dft_2d, fft, fft_2d, FTDirection};
use crate::modifications::Transformation;
use image::{GrayImage, Luma, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;

mod util {
    use std::convert::identity;
    use num::Zero;

    pub fn max<TSource, TResult, TMap>(data: &Vec<Vec<TSource>>, map: TMap) -> TResult
    where
        TMap: Fn(&TSource) -> TResult,
        TResult: PartialOrd + Zero,
    {
        let mut max = TResult::zero();
        for value in data.iter().flat_map(identity) {
            let value = map(value);
            if value > max {
                max = value;
            }
        }
        max
    }
}

pub trait ImageFourierTransform {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>>;
    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>>;
}

pub struct DFT;

impl ImageFourierTransform for DFT {
    fn transform(image: &RgbImage) -> Vec<Vec<Complex<f64>>> {
        assert_pow_2(image.height());
        assert_pow_2(image.width());

        let pixels: Vec<Vec<_>> = to_grayscale(image)
            .rows()
            .into_iter()
            .map(|row| row.map(|Luma([x])| *x as f64 / u8::MAX as f64).collect())
            .collect();

        let pixels_as_slice: Vec<_> = pixels.iter().map(|x| x.as_slice()).collect();

        dft_2d(pixels_as_slice.as_slice(), FTDirection::Forward)
    }

    fn inverse(data: &Vec<Vec<Complex<f64>>>) -> Vec<Vec<Complex<f64>>> {
        let data_as_slice: Vec<_> = data.iter().map(|x| x.as_slice()).collect();
        dft_2d(data_as_slice.as_slice(), FTDirection::Forward)
    }
}

impl Transformation for DFT {
    fn apply(&self, image: &mut RgbImage) {
        let transformed = Self::transform(image);

        let max_value = util::max(&transformed, |x| x.abs());

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

        let pixels: Vec<Vec<_>> = to_grayscale(image)
            .rows()
            .into_iter()
            .map(|row| row.map(|Luma([x])| *x as f64 / u8::MAX as f64).collect())
            .collect();

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

        let max_value = util::max(&transformed, |x| x.abs());

        let magnitude = GrayImage::from_fn(image.width(), image.height(), |x, y| {
            let (x, y) = swap_quadrant_coordinates(x, y, image.width(), image.height());
            normalize(transformed[y as usize][x as usize].abs(), max_value)
        });

        *image = to_rgb(magnitude);
    }
}
