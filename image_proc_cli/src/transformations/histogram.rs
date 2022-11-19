use image::{ImageBuffer, RgbImage};
use image_proc::histogram::Histogram;
use image_proc::modifications::Transformation;

#[derive(Copy, Clone)]
pub enum HistogramChannelOptions {
    R = 0,
    G = 1,
    B = 2,
    All,
}

pub struct HistogramConverter {
    channel: HistogramChannelOptions,
}

impl HistogramConverter {
    pub fn new(channel: HistogramChannelOptions) -> Self {
        Self { channel }
    }
}

const HISTOGRAM_HEIGHT: u32 = 100;

impl Transformation for HistogramConverter {
    fn apply(&self, image: &mut RgbImage) {
        let normalized_histogram = {
            let mut histogram = Histogram::new(image);
            let max_height = *histogram.iter().flat_map(|h| h.iter()).max().unwrap(); // cannot be empty
            for channel in 0..3 {
                let histogram = &mut histogram[channel as usize];
                for x in histogram {
                    *x = ((*x as f64) * HISTOGRAM_HEIGHT as f64 / (max_height as f64)) as u32
                }
            }
            histogram
        };
        match &self.channel {
            HistogramChannelOptions::All => {
                let histogram_image = ImageBuffer::from_fn(256, HISTOGRAM_HEIGHT, |x, y| {
                    let y = HISTOGRAM_HEIGHT - y;
                    let mut pixel = [0u8;3];
                    for channel in 0..3 {
                        let height = normalized_histogram[channel][x as usize];
                        if y <= height {
                            pixel[channel] = 255;
                        }
                    }
                    image::Rgb(pixel)
                });
                *image = histogram_image;
            }
            _ => {
                debug_assert!((self.channel as usize) < 3);
                let normalized_histogram = normalized_histogram[self.channel as usize];
                let histogram_image = ImageBuffer::from_fn(256, HISTOGRAM_HEIGHT, |x, y| {
                    let height = normalized_histogram[x as usize];
                    let y = HISTOGRAM_HEIGHT - y;
                    if y <= height {
                        image::Rgb([255, 255, 255])
                    } else {
                        image::Rgb([0, 0, 0])
                    }
                });
                *image = histogram_image;
            }
        }
    }
}
