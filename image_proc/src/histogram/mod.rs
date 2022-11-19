use std::ops::{Deref, DerefMut};
use image::RgbImage;

pub struct Histogram {
    data: [u32; 256],
}

impl Histogram {
    pub fn new(image: &RgbImage) -> Self {
        let mut data = [0u32; 256];
        for pixel in image.pixels() {
            let luminosity = ((pixel[0] as u32 + pixel[1] as u32 + pixel[2] as u32) / 3) as usize;
            debug_assert!(luminosity < 256);
            data[luminosity] += 1;
        }
        Self { data }
    }
}

impl Deref for Histogram {
    type Target = [u32;256];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Histogram {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}