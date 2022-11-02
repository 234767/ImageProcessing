use image::RgbImage;
use crate::analysis::Analyzer;

pub struct MeanSquareError {}

impl Analyzer for MeanSquareError {
    fn compare(&self, original: &RgbImage, other: &RgbImage) -> Result<String, String> {
        let mut total: [i64; 3] = [0, 0, 0];
        for (x, y, original_pixel) in original.enumerate_pixels() {
            let other_pixel = other.get_pixel(x, y);
            for channel in 1..3 {
                let difference: i64 =
                    (original_pixel[channel] as i64) - (other_pixel[channel] as i64);
                total[channel] += difference * difference;
            }
        }
        let result: f64 = (total.iter().sum::<i64>() as f64)
            / (3.0 * original.width() as f64 * original.height() as f64);
        Ok(format!("MSE: {}", result))
    }
}
