use image::{Rgb, RgbImage};
use num::pow;
/*
(C1) Mean (--cmean). Variance (--cvariance).
(C2) Standard deviation (--cstdev). Variation coefficient I (--cvarcoi).
(C3) Asymmetry coefficient (--casyco).
(C4) Flattening coefficient (--casyco).
(C5) Variation coefficient II (--cvarcoii).
(C6) Information source entropy (--centropy).
*/
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

impl Mean{
    fn analyze(image: &RgbImage) -> f64{
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
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

impl Variance{
    fn analyze(image: &RgbImage) -> f64{
        let mean = Mean::analyze(image);
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        let variance = pow(sum - mean, 2) /(image.width() * image.height() * 3) as f64;
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

impl StandardDeviation{
    fn analyze(image: &RgbImage) -> f64{
        let variance = Variance::analyze(image);
        let std_deviation = f64::sqrt(variance);
        std_deviation
    }
}

impl Characteristic for StandardDeviation{
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let std_deviation = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Standard Deviation:", std_deviation))
    }
}

pub struct VarianceCoefficient1;

impl VarianceCoefficient1{
    fn analyze(image: &RgbImage) -> f64{
        let mean = Mean::analyze(image);
        let std_deviation = StandardDeviation::analyze(image);
        let var_coe_1 = std_deviation/mean;
        var_coe_1
    }
}

impl Characteristic for VarianceCoefficient1 {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let var_coe_1 = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Variance Coefficient I:", var_coe_1))
    }
}

pub struct AsymmetryCoefficient;

impl AsymmetryCoefficient{
    fn analyze(image: &RgbImage) -> f64{
        let mean = Mean::analyze(image);
        let std_deviation = StandardDeviation::analyze(image);
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        let asymmetry = pow(sum - mean, 3)/ pow(std_deviation, 3)*(image.width() * image.height() * 3) as f64;
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

impl FlatteningCoefficient{
    fn analyze(image: &RgbImage) -> f64{
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        let mean = Mean::analyze(image);
        let std_deviation = StandardDeviation::analyze(image);
        let flat = pow(sum - mean, 4)/ pow(std_deviation, 4)*(image.width() * image.height() * 3) as f64 -3.0;
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

impl VarianceCoefficient2{
    fn analyze(image: &RgbImage) -> f64{
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        let var2 = pow(sum,2) / pow(image.width() * image.height() * 3,2) as f64;
        var2
    }
}

impl Characteristic for VarianceCoefficient2 {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let var2 = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Flattening Coefficient:", var2))
    }
}

pub struct InformationSourceEntropy;

impl InformationSourceEntropy{
    fn analyze(image: &RgbImage) -> f64{
        let sum: f64 = image
            .pixels()
            .flat_map(
                |Rgb(pixel)| pixel.iter().map(|x| *x as f64), // converting &[u8;3] to 3 f64s
            )
            .sum();
        let a = sum / (image.width() * image.height() * 3) as f64;
        let info_src_ent = (-1.0 * sum * a.log2())/ (image.width() * image.height() * 3) as f64;
        info_src_ent
    }
}

impl Characteristic for InformationSourceEntropy {
    fn analyze(&self, image: &RgbImage) -> Result<String, String> {
        let info_src_ent = Self::analyze(image);
        Ok(format!("{:10} {:6.3}", "Information source entropy:", info_src_ent))
    }
}