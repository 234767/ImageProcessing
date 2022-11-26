use crate::histogram::Histogram;
use crate::modifications::Transformation;
use image::RgbImage;

///(H3) Raleigh final probability density function (--hraleigh).
pub struct HRaleigh {
    alpha: f64,
    gmin: u8,
}

impl HRaleigh {
    pub fn new(alpha: f64, gmin: u8) -> Self {
        Self { alpha, gmin }
    }
}

impl Transformation for HRaleigh {
    fn apply(&self, image: &mut RgbImage) {
        let partial_sums: [[u32; 256]; 3] = {
            let histogram = Histogram::new(image);
            histogram
                .into_iter()
                .map(|h| {
                    let partial_sums = h
                        .iter()
                        .scan(0u32, |sum, value| {
                            *sum += value;
                            Some(*sum)
                        })
                        .collect::<Vec<u32>>();
                    debug_assert_eq!(partial_sums.len(), 256);
                    partial_sums.try_into().unwrap()
                })
                .collect::<Vec<[u32; 256]>>()
                .try_into()
                .unwrap()
        };

        let image_size = image.width() * image.height();

        let brightness_lookup = {
            let mut brightness_lookup = [[0u8; 256]; 3];
            for channel in 0..3 {
                for i in 0..256 {
                    let log_base = image_size as f64 / partial_sums[channel][i] as f64;
                    let root_base = f64::sqrt(2.0 * self.alpha * self.alpha * f64::ln(log_base));
                    brightness_lookup[channel][i] = self.gmin + f64::clamp(root_base,0.0,255.0) as u8;
                }
            }
            brightness_lookup
        };

        for pixel in image.pixels_mut() {
            for channel in 0..3 {
                let luminosity = pixel[channel];
                let new_luminosity = brightness_lookup[channel][luminosity as usize];
                pixel[channel] = new_luminosity
            }
        }
    }
}
