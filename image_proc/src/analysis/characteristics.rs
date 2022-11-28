use crate::histogram::Histogram;
use image::RgbImage;
use num::pow::Pow;

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

impl Mean {
    fn analyze(image: &RgbImage) -> f64 {
        let histogram = Histogram::new(image);
        let mut sum: f64 = 0.0;
        for channel in 0..3 {
            for luma in 0..=255 {
                sum += luma as f64 * histogram[channel][luma] as f64;
            }
        }
        let mean = sum / (image.width() * image.height() * 3) as f64;
        mean
    }
}

impl Characteristic for Mean {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let mean = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Mean:", mean))
    }
}

pub struct Variance;

impl Variance {
    fn analyze(image: &RgbImage) -> f64 {
        let histogram = Histogram::new(image);
        let mean = Mean::analyze(image);
        let mut sum: f64 = 0.0;
        for channel in 0..3 {
            for luma in 0..=255 {
                sum += f64::pow(luma as f64 - mean, 2.0) * histogram[channel][luma] as f64;
            }
        }
        let variance = sum / (image.width() * image.height() * 3) as f64;
        variance
    }
}

impl Characteristic for Variance {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let variance = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Variance:", variance))
    }
}

pub struct StandardDeviation;

impl StandardDeviation {
    fn analyze(image: &RgbImage) -> f64 {
        let variance = Variance::analyze(image);
        let std_deviation = f64::sqrt(variance);
        std_deviation
    }
}

impl Characteristic for StandardDeviation {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let std_deviation = Self::analyze(image);
        Ok(format!(
            "{:10} {:6.3}",
            "Standard Deviation:", std_deviation
        ))
    }
}

pub struct VarianceCoefficient1;

impl VarianceCoefficient1 {
    fn analyze(image: &RgbImage) -> f64 {
        let mean = Mean::analyze(image);
        let std_deviation = StandardDeviation::analyze(image);
        let var_coe_1 = std_deviation / mean;
        var_coe_1
    }
}

impl Characteristic for VarianceCoefficient1 {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let var_coe_1 = Self::analyze(image);
        Ok(format!(
            "{:10} {:6.3}",
            "Variance Coefficient I:", var_coe_1
        ))
    }
}

pub struct AsymmetryCoefficient;

impl AsymmetryCoefficient {
    fn analyze(image: &RgbImage) -> f64 {
        let histogram = Histogram::new(image);
        let mean = Mean::analyze(image);
        let std_deviation = StandardDeviation::analyze(image);
        let mut sum: f64 = 0.0;
        for channel in 0..3 {
            for luma in 0..=255 {
                sum += f64::pow(luma as f64 - mean, 3.0) * histogram[channel][luma] as f64;
            }
        }
        let asymmetry = sum / (f64::pow(std_deviation, 3) * (image.width() * image.height() * 3) as f64);
        asymmetry
    }
}

impl Characteristic for AsymmetryCoefficient {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let asymmetry = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Asymmetry Coeficcient:", asymmetry))
    }
}

pub struct FlatteningCoefficient;

impl FlatteningCoefficient {
    fn analyze(image: &RgbImage) -> f64 {
        let histogram = Histogram::new(image);
        let mean = Mean::analyze(image);
        let std_deviation = StandardDeviation::analyze(image);
        let mut sum: f64 = 0.0;
        for channel in 0..3 {
            for luma in 0..=255 {
                sum += f64::pow(luma as f64 - mean, 4.0) * histogram[channel][luma] as f64 - 3.0;
            }
        }
        let flat = sum / (f64::pow(std_deviation, 4) * (image.width() * image.height() * 3) as f64);
        flat
    }
}

impl Characteristic for FlatteningCoefficient {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let flat = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Flattening Coefficient:", flat))
    }
}

pub struct VarianceCoefficient2;

impl VarianceCoefficient2 {
    fn analyze(image: &RgbImage) -> f64 {
        let histogram = Histogram::new(image);
        let mut sum: f64 = 0.0;
        for channel in 0..3 {
            for luma in 0..=255 {
                sum += f64::pow(histogram[channel][luma] as f64, 2) as f64;
            }
        }
        let image_size = image.width() * image.height();
        let n2 = f64::pow(image_size as f64, 2);
        let var2 = sum / (n2 * 3.0);
        var2
    }
}

impl Characteristic for VarianceCoefficient2 {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let var2 = Self::analyze(image);
        Ok(format!("{:10} {:6.6}", "Flattening Coefficient:", var2))
    }
}

pub struct InformationSourceEntropy;

impl InformationSourceEntropy {
    fn analyze(image: &RgbImage) -> f64 {
        let n = (image.width() * image.height()) as f64;
        let histogram = Histogram::new(image);
        let mut sum: f64 = 0.0;
        for channel in 0..3 {
            for luma in 0..=255 {
                let num_pixels = histogram[channel][luma];
                if num_pixels == 0 {
                    continue;
                }
                sum += num_pixels as f64 * f64::log2(num_pixels as f64 / n);
            }
        }
        let info_src_ent = -1.0 * sum / (n * 3.0) as f64;
        info_src_ent
    }
}

impl Characteristic for InformationSourceEntropy {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let info_src_ent = Self::analyze(image);
        Ok(format!(
            "{:10} {:6.6}",
            "Information source entropy:", info_src_ent
        ))
    }
}

#[cfg(test)]
mod tests;
