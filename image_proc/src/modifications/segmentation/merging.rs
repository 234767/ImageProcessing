use std::collections::vec_deque::VecDeque;
use image::{GrayImage, ImageBuffer, Luma, Rgb};
use crate::modifications::Transformation;

pub struct RegionGrowing {
    seed_x: u32,
    seed_y: u32,
    tolerance: u8,
}

impl RegionGrowing {
    pub fn new(seed_x: u32, seed_y: u32, tolerance: u8) -> Result<Self, String> {
        // if seed_x >= image.width() || seed_y >= image.height() {
        //     return Err(format!("Seed point ({}, {}) is outside of the image dimensions ({}, {})", seed_x, seed_y, image.width(), image.height()));
        // }
        Ok(Self { seed_x, seed_y, tolerance })
    }
}

impl Transformation for RegionGrowing {
    fn apply(&self, image: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let mut new_image: GrayImage = ImageBuffer::new(image.width(), image.height());
        let mut queue = VecDeque::new();
        queue.push_back((self.seed_x, self.seed_y));
        let seed_pixel = image.get_pixel(self.seed_x, self.seed_y);
        while let Some((x, y)) = queue.pop_front() {
            if x >= image.width() || y >= image.height() {
                continue;
            }
            let pixel = image.get_pixel(x as u32, y as u32);
            let new_image_pixel = new_image.get_pixel(x as u32, y as u32);
            if new_image_pixel[0] == 0 && is_similar(pixel, seed_pixel , self.tolerance) {
                new_image.put_pixel(x as u32, y as u32, Luma([255]));
                queue.push_back((x + 1, y));
                queue.push_back((x, y + 1));
                queue.push_back((x - 1, y));
                queue.push_back((x, y - 1));
            }
        }
        //*image = new_image;
    }
}

fn is_similar(pixel: &Rgb<u8>, seed_pixel: &Rgb<u8>, tolerance: u8) -> bool {
    (pixel[0] as i32 - seed_pixel[0] as i32).abs() <= tolerance as i32
}

