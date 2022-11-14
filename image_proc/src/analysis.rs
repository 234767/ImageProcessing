use image::RgbImage;

pub mod analyzers;
mod util;

pub use analyzers::*;

pub trait Analyzer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String>;
}

pub struct CompositeAnalyzer {
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
