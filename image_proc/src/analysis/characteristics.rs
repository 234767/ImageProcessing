use image::{Rgb, RgbImage};

pub trait Characteristic {
    fn analyze(&self, image: &RgbImage) -> Result<String, String>;
}

pub struct CompositeCharacteristic {
    pub characteristics: Vec<Box<dyn Characteristic>>,
}

impl CompositeCharacteristic {
    pub fn new() -> Self {
        Self {
            characteristics: Vec::new(),
        }
    }
}

impl Characteristic for CompositeCharacteristic {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let mut result = String::new();
        for characteristic in &self.characteristics {
            result.push_str(&characteristic.analyze(image)?);
            result.push_str("\n");
        }
        Ok(result)
    }
}
