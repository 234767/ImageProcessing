use super::util::{map_and_reduce, map_and_sum};
use image::{Rgb, RgbImage};
use std::cmp::max;

pub trait ImageComparer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String>;
}

pub struct CompositeComparer {
    pub analyzers: Vec<Box<dyn ImageComparer>>,
}

impl CompositeComparer {
    pub fn new() -> Self {
        CompositeComparer {
            analyzers: Vec::new(),
        }
    }
}

impl ImageComparer for CompositeComparer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let mut result = String::new();
        for analyzer in &self.analyzers {
            result.push_str(&analyzer.compare(original, modified)?);
            result.push_str("\n");
        }
        Ok(result)
    }
}

pub struct MeanSquareError {}

impl MeanSquareError {
    fn compare(original: &RgbImage, modified: &RgbImage) -> f64 {
        let Rgb(totals) = map_and_sum(original, modified, |a, b| {
            let difference = a as i128 - b as i128;
            difference * difference
        });
        let result: f64 = (totals.iter().sum::<i128>() as f64)
            / (3.0 * original.width() as f64 * original.height() as f64);
        result
    }
}

impl ImageComparer for MeanSquareError {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result = Self::compare(original, modified);
        Ok(format!("{:10} {:6.3}", "MSE:", result))
    }
}

/// Peak mean-square-error
pub struct PMSE {}

impl PMSE {
    fn compare(original: &RgbImage, modified: &RgbImage) -> f64 {
        let mse = MeanSquareError::compare(original, modified);
        let max_luminance = u8::MAX as f64;

        mse / max_luminance
    }
}

impl ImageComparer for PMSE {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result = Self::compare(original, modified);
        Ok(format!("{:10} {:6.3}", "Peak MSE:", result))
    }
}

pub struct MaximumDifference {}

impl MaximumDifference {
    fn compare(original: &RgbImage, modified: &RgbImage) -> i128 {
        let Rgb(results) = map_and_reduce(
            original,
            modified,
            |old, new| old as i128 - new as i128,
            max,
            Rgb([0, 0, 0]),
        );

        *results.iter().max().unwrap() // iterator cannot be empty, so it is safe to call unwrap()
    }
}

impl ImageComparer for MaximumDifference {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result = Self::compare(original, modified);
        Ok(format!("{:3} {:9}", "MD:", result))
    }
}

/// Signal-to-noise ratio
pub struct SNR {}

impl SNR {
    fn compare(original: &RgbImage, modified: &RgbImage) -> f64 {
        let Rgb(luminance_sums) =
            map_and_sum(original, modified, |old, _new| (old as i128 * old as i128));
        let mean_luminance = (luminance_sums.iter().sum::<i128>() as f64)
            / ((3 * original.width() * original.height()) as f64);

        let mse = MeanSquareError::compare(original, modified);

        10f64 * f64::log10(mean_luminance / mse)
    }
}

impl ImageComparer for SNR {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result = Self::compare(original, modified);
        Ok(format!("{:10} {:6.3}", "SNR:", result))
    }
}

/// Peak signal-to-noise ratio
pub struct PSNR {}

impl PSNR {
    fn compare(original: &RgbImage, modified: &RgbImage) -> f64 {
        let max_luminance = u8::MAX as f64 * u8::MAX as f64;
        let mse = MeanSquareError::compare(original, modified);

        10f64 * f64::log10(max_luminance / mse)
    }
}

impl ImageComparer for PSNR {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result = Self::compare(original, modified);
        Ok(format!("{:10} {:6.3}", "Peak SNR:", result))
    }
}
