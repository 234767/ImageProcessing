use crate::modifications::Transformation;
use image::{DynamicImage, GrayImage, ImageBuffer, Luma, Rgb, RgbImage};
use std::cell::Cell;
use std::collections::vec_deque::VecDeque;

pub struct RegionGrowing {
    seed_x: Cell<u32>,
    seed_y: Cell<u32>,
    tolerance: u8,
}

impl RegionGrowing {
    pub fn new(seed_x: u32, seed_y: u32, tolerance: u8) -> Self {
        Self {
            seed_x: Cell::new(seed_x),
            seed_y: Cell::new(seed_y),
            tolerance,
        }
    }
}

impl Transformation for RegionGrowing {
    fn apply(&self, image: &mut RgbImage) {
        if self.seed_x.get() >= image.width() {
            self.seed_x.set(image.width() - 1);
        }
        if self.seed_y.get() >= image.height() {
            self.seed_y.set(image.height() - 1);
        }
        let seed_pixel = image.get_pixel(self.seed_x.get(), self.seed_y.get());
        let mut new_image: GrayImage = ImageBuffer::new(image.width(), image.height());
        let mut queue = VecDeque::new();
        queue.push_back((self.seed_x.get(), self.seed_y.get()));

        while let Some((x, y)) = queue.pop_front() {
            if x >= image.width() || y >= image.height() {
                continue;
            }
            let pixel = image.get_pixel(x as u32, y as u32);
            let new_image_pixel = new_image.get_pixel(x as u32, y as u32);

            if new_image_pixel[0] == 0 && is_similar(pixel, seed_pixel, self.tolerance) {
                new_image.put_pixel(x as u32, y as u32, Luma([255]));

                queue.push_back((x + 1, y));
                queue.push_back((x, y + 1));
                queue.push_back((x - 1, y));
                queue.push_back((x, y - 1));
            }
        }

        *image = DynamicImage::from(new_image).to_rgb8();
    }
}

fn is_similar(pixel: &Rgb<u8>, seed_pixel: &Rgb<u8>, tolerance: u8) -> bool {
    (pixel[0] as i32 - seed_pixel[0] as i32) <= tolerance as i32
}
