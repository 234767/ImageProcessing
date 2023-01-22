use std::cmp::Ordering::*;
use super::{
    image_fourier_transforms::{ImageFourierTransform, FFT},
    util::*,
};
use crate::modifications::{is_edge, Transformation};
use image::{GrayImage, Luma, RgbImage};
use num::complex::ComplexFloat;
use std::convert::identity;

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
        let mask = move |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            let distance_squared = x*x + y*y;
            match (distance_squared.cmp(&from_squared), distance_squared.cmp(&to_squared)) {
                (Less, _) => 0.0,
                (_, Greater) => 0.0,
                (_,_) => 1.0
            }
        };
        apply_mask_filter::<FFT, _>(image, &mask);
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
        let mask = move |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            let distance_squared = x*x + y*y;
            match (distance_squared.cmp(&from_squared), distance_squared.cmp(&to_squared)) {
                (Less, _) => 1.0,
                (_, Greater) => 1.0,
                (_,_) => 0.0
            }
        };
        apply_mask_filter::<FFT, _>(image, &mask);
    }
}

//(F5) High-pass filter with detection of edge direction
pub struct HighPassEdgeFilter{
    radius: u32,
    direction: EdgeDirection,

}
pub enum EdgeDirection {
    X,
    Y,
    Both
}

impl HighPassEdgeFilter{
    pub fn new(radius: u32, direction: EdgeDirection) -> Self {
        Self { radius, direction }
    }
}

impl Transformation for HighPassEdgeFilter{
    fn apply(&self, image: &mut RgbImage) {
        let sobel_operator = SobelOperatorEdge {};
        let (dx, dy) = sobel_operator.apply(image);

        let half_width = image.width() / 2;
        let half_height = image.height() / 2;
        let radius_squared = self.radius * self.radius;
        let mask = |x: u32, y: u32| -> f64 {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            let distance_squared = x * x + y * y;
            match self.direction {
                EdgeDirection::X => dx[(y * image.width() + x) as usize] as f64,
                EdgeDirection::Y => dy[(y * image.width() + x) as usize] as f64,
                EdgeDirection::Both => {
                    if distance_squared > radius_squared {
                        1.0
                    } else {
                        0.0
                    }
                }
            }
        };
        //apply_mask_filter::<FFT, _>(image, &mask);
    }
}

//(F6) Phase modifying filter
pub struct PhaseFilter{
    angle: f64,
}

impl PhaseFilter{
    pub fn new(angle: f64) -> Self {
        Self { angle }
    }
}

impl Transformation for PhaseFilter{
    fn apply(&self, image: &mut RgbImage) {
        let half_width = image.width() / 2;
        let half_height = image.height() / 2;
        let mask = move |x: u32, y: u32| {
            let x = u32::abs_diff(x, half_width);
            let y = u32::abs_diff(y, half_height);
            let angle = (y as f64).atan2(x as f64) + self.angle;
            //Complex::new(angle.cos(), angle.sin()).re
            angle
        };
        apply_mask_filter::<FFT, _>(image, &mask);
    }
}

//Didnt want to change the sobel from previous exercise, so we have a new different one.
//In my implementation SobelOperator is modifying the image in place -> instead of returning the gradient images (dx, dy) that HighPassEdgeFilter expects
//We can change it in future, but didnt want to destroy the code for now

pub struct SobelOperatorEdge;

impl SobelOperatorEdge {
    fn gradient(&self, image: &RgbImage, x: u32, y: u32) -> (f64, f64) {
        let mut sobel_x = 0.0;
        let mut sobel_y = 0.0;
        for channel in 0..3 {
            sobel_x += image.get_pixel(x + 1, y - 1)[channel] as f64
                + 2.0 * image.get_pixel(x + 1, y)[channel] as f64
                + image.get_pixel(x + 1, y + 1)[channel] as f64
                - (image.get_pixel(x - 1, y - 1)[channel] as f64
                + 2.0 * image.get_pixel(x - 1, y)[channel] as f64
                + image.get_pixel(x - 1, y + 1)[channel] as f64);
            sobel_y += image.get_pixel(x - 1, y - 1)[channel] as f64
                + 2.0 * image.get_pixel(x, y - 1)[channel] as f64
                + image.get_pixel(x + 1, y - 1)[channel] as f64
                - (image.get_pixel(x - 1, y + 1)[channel] as f64
                + 2.0 * image.get_pixel(x, y + 1)[channel] as f64
                + image.get_pixel(x + 1, y + 1)[channel] as f64);
        }
        (sobel_x, sobel_y)
    }
    fn apply(&self, image: &RgbImage) -> (Vec<f32>, Vec<f32>) {
        let mut dx = vec![0.0; (image.width() * image.height()) as usize];
        let mut dy = vec![0.0; (image.width() * image.height()) as usize];
        for (x, y, _) in image.enumerate_pixels() {
            if is_edge(image, x, y) {
                continue;
            }
            let (sobel_x, sobel_y) = self.gradient(image, x, y);
            dx[(y * image.width() + x) as usize] = sobel_x as f32;
            dy[(y * image.width() + x) as usize] = sobel_y as f32;
        }
        (dx, dy)
    }
}