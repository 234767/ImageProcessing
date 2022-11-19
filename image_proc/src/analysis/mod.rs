use image::RgbImage;

pub mod comparers;
pub mod characteristics;
mod util;

pub use comparers::*;

pub trait ImageComparer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String>;
}

pub struct CompositeAnalyzer {
    pub analyzers: Vec<Box<dyn ImageComparer>>,
}

impl CompositeAnalyzer {
    pub fn new() -> Self {
        CompositeAnalyzer {
            analyzers: Vec::new(),
        }
    }
}

impl ImageComparer for CompositeAnalyzer {
    fn compare(&self, original: &RgbImage, modified: &RgbImage) -> Result<String, String> {
        let mut result = String::new();
        for analyzer in &self.analyzers {
            result.push_str(&analyzer.compare(original, modified)?);
            result.push_str("\n");
        }
        Ok(result)
    }
}
