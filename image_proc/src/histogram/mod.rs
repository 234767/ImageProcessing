mod improvement;

use image::RgbImage;
use std::ops::{Deref, DerefMut};

pub struct Histogram {
    data: [[u32; 256]; 3],
}

impl Histogram {
    pub fn new(image: &RgbImage) -> Self {
        let mut data = [[0u32; 256]; 3];
        for pixel in image.pixels() {
            for channel in 0..3 {
                let luminosity = pixel[channel];
                data[channel][luminosity as usize] += 1;
            }
        }
        Self { data }
    }
}

impl Deref for Histogram {
    type Target = [[u32; 256]; 3];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Histogram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
