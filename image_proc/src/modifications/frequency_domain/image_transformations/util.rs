use image::{DynamicImage, GrayImage, Luma, RgbImage};
use num::Zero;
use std::convert::identity;

pub fn to_grayscale(image: &RgbImage) -> GrayImage {
    image::imageops::grayscale(image)
}

pub fn to_rgb(image: GrayImage) -> RgbImage {
    DynamicImage::from(image).to_rgb8()
}

pub fn normalize(value: f64, max_value: f64) -> Luma<u8> {
    let normalization_factor = u8::MAX as f64 / f64::ln(1.0 + max_value);
    let value = (normalization_factor * f64::ln(1.0 + value)).clamp(0.0, u8::MAX as f64);
    Luma([value as u8])
}

pub fn assert_pow_2(num: u32) {
    let log = (num as f64).log2();
    assert!(log - log.floor() < 1e-5, "Number must be a power of 2");
}

pub fn swap_quadrant_coordinates(x: u32, y: u32, width: u32, height: u32) -> (u32, u32) {
    use core::cmp::Ordering::*;
    match (x.cmp(&(width / 2)), y.cmp(&(height / 2))) {
        (Less, Less) => (x + width / 2, y + height / 2),
        (Less, _) => (x + width / 2, y - height / 2),
        (_, Less) => (x - width / 2, y + height / 2),
        (_, _) => (x - width / 2, y - height / 2),
    }
}

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

pub fn image_to_matrix(image: &RgbImage) -> Vec<Vec<f64>> {
    to_grayscale(image)
        .rows()
        .into_iter()
        .map(|row| row.map(|Luma([x])| *x as f64 / u8::MAX as f64).collect())
        .collect()
}
