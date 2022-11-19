use image::RgbImage;

pub mod comparers;
pub mod characteristics;
mod util;

pub use comparers::*;

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
