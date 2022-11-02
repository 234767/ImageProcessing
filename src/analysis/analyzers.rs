use crate::analysis::util::{map_and_reduce, map_and_sum};
use crate::analysis::Analyzer;
use image::{Rgb, RgbImage};
use std::cmp::max;

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

pub struct MaximumDifference {}

impl Analyzer for MaximumDifference {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let Rgb(results) = map_and_reduce(
            original,
            modified,
            |old, new| old as i128 - new as i128,
            max,
            Rgb([0, 0, 0]),
        );

        let result_opt = results.iter().max();
        match result_opt {
            Some(result) => Ok(format!("MD: {}", result)),
            None => Err(String::from("Could not compute maximum difference"))
        }
    }
}
