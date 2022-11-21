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

pub struct Mean;

impl Characteristic for Mean {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        let mean = sum / (image.width() * image.height() * 3) as f64;
        Ok(format!("{:10} {:6.3}", "Mean:", mean))
    }
}
/*
pub struct Variance;

impl Characteristic for Variance {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        //let variance =
        Ok(format!("{:10} {:6.3}", "Variance:", variance))
    }
}

 */
