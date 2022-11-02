use crate::parsing::Args;
use image::{Rgb, RgbImage};
use num::Num;

use analyzers::*;

mod analyzers;

pub trait Analyzer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String>;
}

pub fn get_analyzers(args: &Args) -> Box<dyn Analyzer> {
    let mut composite = CompositeAnalyzer::new();
    if args.args.contains_key("--mse") {
        composite.analyzers.push(Box::new(MeanSquareError {}));
    }
    return Box::new(composite);
}

struct CompositeAnalyzer {
    analyzers: Vec<Box<dyn Analyzer>>,
}

impl CompositeAnalyzer {
    pub fn new() -> Self {
        CompositeAnalyzer {
            analyzers: Vec::new(),
        }
    }
}

impl Analyzer for CompositeAnalyzer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let mut result = String::new();
        for analyzer in &self.analyzers {
            result.push_str(&analyzer.compare(original, modified)?);
        }
        Ok(result)
    }
}

/// For each channel of RGB maps a given function, and sums the results
///
/// # Arguments
///
/// * `original`: original image
/// * `modified`: modified image
/// * `function`: function to map the different brightness values
///
/// returns: Rgb<i128>
/// Values for RGB channels respectively
fn map_and_sum<F>(original: &RgbImage, modified: &RgbImage, function: F) -> Rgb<i128>
where
    F: Fn(u8, u8) -> i128,
{
    let mut total: [i128; 3] = [0, 0, 0];
    for (x, y, original_pixel) in original.enumerate_pixels() {
        let modified_pixel = modified.get_pixel(x, y);
        for channel in 1..3 {
            let value = function(original_pixel[channel], modified_pixel[channel]);
            total[channel] += value;
        }
    }
    Rgb(total)
}
