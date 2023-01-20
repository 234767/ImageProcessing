use crate::modifications::{
    frequency_domain::fourier_transform::{dft_2d, FFTDirection},
    Transformation,
};
use image::{DynamicImage, GrayImage, Luma, Rgb, RgbImage};
use num::{complex::ComplexFloat, pow::Pow, Complex};
use std::convert::identity;

fn to_grayscale(image: &RgbImage) -> GrayImage {
    image::imageops::grayscale(image)
}

fn to_rgb(image: GrayImage) -> RgbImage {
    DynamicImage::from(image).to_rgb8()
}

fn complex_to_pixel(value: &Complex<f64>, max_value: f64) -> Luma<u8> {
    let normalization_factor = u8::MAX as f64 / f64::ln(1.0 + max_value);
    let magnitude = f64::sqrt(value.re.pow(2) + value.im.pow(2));
    let value = (normalization_factor * f64::ln(1.0 + magnitude)).clamp(0.0, u8::MAX as f64);
    Luma([value as u8])
}

fn assert_pow_2(num: &u32) {
    let log = (*num as f64).log2();
    assert!(log - log.floor() < 1e-5, "Number must be a power of 2");
}

fn swap_image_parts(image: &mut RgbImage) {
    let w = image.width();
    let h = image.height();

    for x in 0..w / 2 {
        for y in 0..h / 2 {
            unsafe {
                let first_pixel = image.get_pixel(x, y) as *const Rgb<u8>;
                let second_pixel = image.get_pixel(x + w / 2, y + h / 2) as *const Rgb<u8>;
                std::ptr::swap(first_pixel as *mut Rgb<u8>, second_pixel as *mut Rgb<u8>);

                let first_pixel = image.get_pixel(x, y + h / 2) as *const Rgb<u8>;
                let second_pixel = image.get_pixel(x + w / 2, y) as *const Rgb<u8>;
                std::ptr::swap(first_pixel as *mut Rgb<u8>, second_pixel as *mut Rgb<u8>);
            }
        }
    }
}

pub struct DFT;

impl Transformation for DFT {
    fn apply(&self, image: &mut RgbImage) {
        assert_pow_2(&image.height());
        assert_pow_2(&image.width());

        let pixels: Vec<Vec<_>> = to_grayscale(image)
            .rows()
            .into_iter()
            .map(|row| row.map(|Luma([x])| *x as f64 / u8::MAX as f64).collect())
            .collect();

        let pixels_as_slice: Vec<_> = pixels.iter().map(|x| x.as_slice()).collect();

        let transformed = dft_2d(pixels_as_slice.as_slice(), FFTDirection::Forward);

        let max_value = {
            let mut max = 0.0;
            for value in transformed.iter().flat_map(identity) {
                if value.abs() > max {
                    max = value.abs();
                }
            }
            max
        };

        let result = GrayImage::from_fn(image.width(), image.height(), |x, y| {
            complex_to_pixel(&transformed[y as usize][x as usize], max_value)
        });

        *image = to_rgb(result);
        swap_image_parts(image);
    }
}
