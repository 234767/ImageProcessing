use crate::analysis::util::{map_and_reduce, map_and_sum};
use crate::analysis::Analyzer;
use image::{Rgb, RgbImage};
use std::cmp::max;

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

impl Analyzer for MeanSquareError {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result = Self::compare(original, modified);
        Ok(format!("MSE: {}", result))
    }
}

pub struct MaximumDifference {}

impl MaximumDifference {
    fn compare(original: &RgbImage, modified: &RgbImage) -> Option<i128> {
        let Rgb(results) = map_and_reduce(
            original,
            modified,
            |old, new| old as i128 - new as i128,
            max,
            Rgb([0, 0, 0]),
        );

        results.iter().max().map(|c| c.clone())
    }
}

impl Analyzer for MaximumDifference {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let result_opt = Self::compare(original, modified);
        match result_opt {
            Some(result) => Ok(format!("MD: {}", result)),
            None => Err(String::from("Could not compute maximum difference"))
        }
    }
}
