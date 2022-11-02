use crate::analysis::Analyzer;
use image::{Rgb, RgbImage};
use crate::analysis::util::map_and_sum;

pub struct MeanSquareError {}

impl Analyzer for MeanSquareError {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let Rgb(totals) = map_and_sum(original, modified, |a, b| {
            let difference = a as i128 - b as i128;
            difference * difference
        });
        let result: f64 = (totals.iter().sum::<i128>() as f64)
            / (3.0 * original.width() as f64 * original.height() as f64);
        Ok(format!("MSE: {}", result))
    }
}