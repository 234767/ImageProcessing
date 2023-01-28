use super::{
    image_fourier_transforms::{ImageFourierTransform, FFT},
    util::*,
};
use crate::modifications::Transformation;
use image::{GrayImage, Luma, Rgb, RgbImage};
use num::complex::ComplexFloat;
use num::Complex;
use std::cmp::Ordering::*;
use std::convert::identity;
use std::f64::consts::PI;
use std::ops::Mul;

mod debug_utils {
    use super::*;
    use num::Complex;

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

fn apply_mask_filter<TFourier, TMask, TMaskResult>(image: &mut RgbImage, mask: &TMask)
where
    TFourier: ImageFourierTransform,
    TMask: Fn(u32, u32) -> TMaskResult,
    TMaskResult: Mul<Complex<f64>, Output = Complex<f64>>,
{
    let mut transform = TFourier::transform(image);
    for (y, row) in transform.iter_mut().enumerate() {
        for (x, pixel) in row.iter_mut().enumerate() {
            let (x, y) =
                swap_quadrant_coordinates(x as u32, y as u32, image.width(), image.height());
            if x == image.width() / 2 && y == image.height() / 2 {
                continue;
            }
            let mask_value = mask(x as u32, y as u32);
            *pixel = mask_value * (*pixel);
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
}

//(F1) Low-pass filter (high-cut filter)
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
        let mask = |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            if x * x + y * y <= radius_squared {
                1.0
            } else {
                0.0
            }
        };
        apply_mask_filter::<FFT, _, _>(image, &mask);
    }
}

//(F2) High-pass filter (low-cut filter)
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
        let mask = |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            if x * x + y * y > radius_squared {
                1.0
            } else {
                0.0
            }
        };
        apply_mask_filter::<FFT, _, _>(image, &mask);
    }
}

//(F3) Band-pass filter
pub struct BandPassFilter {
    from_radius: u32,
    to_radius: u32,
}

impl BandPassFilter {
    pub fn new(from: u32, to: u32) -> Self {
        assert!(from <= to);
        Self {
            from_radius: from,
            to_radius: to,
        }
    }
}

impl Transformation for BandPassFilter {
    fn apply(&self, image: &mut RgbImage) {
        let from_squared = self.from_radius.pow(2);
        let to_squared = self.to_radius.pow(2);
        let half_width = image.width() / 2;
        let half_height = image.height() / 2;
        let mask = |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            let distance_squared = x * x + y * y;
            match (
                distance_squared.cmp(&from_squared),
                distance_squared.cmp(&to_squared),
            ) {
                (Less, _) => 0.0,
                (_, Greater) => 0.0,
                (_, _) => 1.0,
            }
        };
        apply_mask_filter::<FFT, _, _>(image, &mask);
    }
}

//(F4) Band-cut filter
pub struct BandCutFilter {
    from_radius: u32,
    to_radius: u32,
}

impl BandCutFilter {
    pub fn new(from: u32, to: u32) -> Self {
        assert!(from <= to);
        Self {
            from_radius: from,
            to_radius: to,
        }
    }
}

impl Transformation for BandCutFilter {
    fn apply(&self, image: &mut RgbImage) {
        let from_squared = self.from_radius.pow(2);
        let to_squared = self.to_radius.pow(2);
        let half_width = image.width() / 2;
        let half_height = image.height() / 2;
        let mask = |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            let distance_squared = x * x + y * y;
            match (
                distance_squared.cmp(&from_squared),
                distance_squared.cmp(&to_squared),
            ) {
                (Less, _) => 1.0,
                (_, Greater) => 1.0,
                (_, _) => 0.0,
            }
        };
        apply_mask_filter::<FFT, _, _>(image, &mask);
    }
}

//(F5) High-pass filter with detection of edge direction
pub struct HighPassFilterWithEdgeDetection {
    mask: GrayImage,
}

impl HighPassFilterWithEdgeDetection {
    pub fn new(mask: GrayImage) -> Self {
        Self { mask }
    }
}

impl Transformation for HighPassFilterWithEdgeDetection {
    fn apply(&self, image: &mut RgbImage) {

        if image.width() != self.mask.width() || image.height() != self.mask.height() {
            panic!("Image is different size than mask");
        }

        let mask = |x: u32, y: u32| {
            let &Luma([luma]) = self.mask.get_pixel(x,y);
            if luma > 128 {
                1.0
            } else {
                0.0
            }
        };

        apply_mask_filter::<FFT, _, _>(image, &mask);

        // Edge detection
        let mut edges = RgbImage::new(image.width(), image.height());
        for x in 1..image.width() - 1 {
            for y in 1..image.height() - 1 {
                let _pixel = image.get_pixel(x, y);
                let (dx, dy) = (
                    (image.get_pixel(x + 1, y)[0] as f32 - image.get_pixel(x - 1, y)[0] as f32),
                    (image.get_pixel(x, y + 1)[0] as f32 - image.get_pixel(x, y - 1)[0] as f32),
                );
                let edge = (dx.powi(2) + dy.powi(2)).sqrt().round() as u8;
                edges.put_pixel(x, y, Rgb([edge, edge, edge]));
            }
        }

        *image = edges;
    }
}

//(F6) Phase modifying filter
pub struct PhaseFilter {
    k: f64,
    l: f64,
}

impl PhaseFilter {
    pub fn new(k: f64, l: f64) -> Self {
        Self { k, l }
    }
}

impl Transformation for PhaseFilter {
    fn apply(&self, image: &mut RgbImage) {
        let height = image.height() as f64;
        let width = image.width() as f64;
        let mask = |x, y| {
            Complex::from_polar(
                1.0,
                -1.0 * (x as f64 * self.k * 2.0 * PI) / height
                    + -1.0 * (y as f64 * self.l * 2.0 * PI) / width
                    + (self.k + self.l) * PI,
            )
        };
        let mut image_clone = image.clone();
        apply_mask_filter::<FFT, _, _>(&mut image_clone, &mask);
        *image = image_clone;
    }
}
