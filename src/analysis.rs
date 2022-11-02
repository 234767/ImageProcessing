use crate::parsing::Args;
use image::RgbImage;


use analyzers::*;

mod analyzers;
mod util;

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
