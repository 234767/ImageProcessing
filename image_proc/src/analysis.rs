use crate::parsing::Args;
use image::RgbImage;
use std::collections::HashMap;

use analyzers::*;

mod analyzers;
mod util;

pub trait Analyzer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String>;
}

pub fn get_analyzers(args: &Args) -> Box<dyn Analyzer> {
    let args: &HashMap<String, String> = &args.args;
    let mut composite = CompositeAnalyzer::new();

    macro_rules! add_if_contains {
        ($key:literal,$object:expr) => {
            if (args.contains_key($key)) {
                composite.analyzers.push(Box::new($object));
            }
        };
    }

    add_if_contains!("--mse", MeanSquareError {});
    add_if_contains!("--pmse", PMSE {});
    add_if_contains!("--snr", SNR {});
    add_if_contains!("--psnr", PSNR {});
    add_if_contains!("--md", MaximumDifference {});

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
            result.push_str("\n");
        }
        Ok(result)
    }
}
