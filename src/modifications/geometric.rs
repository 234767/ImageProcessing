use crate::modifications::Transformation;
use image::{ImageBuffer, RgbImage};

pub struct HorizontalFlip {}

impl Transformation for HorizontalFlip {
    fn apply(&self, image: &mut RgbImage) {
        image::imageops::flip_horizontal(image);
        let width = image.width();
        for y in 1..image.height() {
            for x in 1..(width / 2) {
                let left_pixel = *image.get_pixel(x, y);
                image.put_pixel(x, y, *image.get_pixel(width - x, y));
                image.put_pixel(width - x, y, left_pixel);
            }
        }
    }
}

pub struct VerticalFlip {}

impl VerticalFlip {
    fn apply(image: &mut RgbImage) {
        let height = image.height();
        for y in 1..(height / 2) {
            for x in 1..image.width() {
                let top_pixel = *image.get_pixel(x, y);
                image.put_pixel(x, y, *image.get_pixel(x, height - y));
                image.put_pixel(x, height - y, top_pixel);
            }
        }
    }
}

impl Transformation for VerticalFlip {
    fn apply(&self, image: &mut RgbImage) {
        VerticalFlip::apply(image)
    }
}

pub struct DiagonalFlip {}

impl Transformation for DiagonalFlip {
    fn apply(&self, image: &mut RgbImage) {
        HorizontalFlip {}.apply(image);
        VerticalFlip {}.apply(image);
    }
}

pub struct Enlarge {
    pub amount: f64,
}

impl Enlarge {
    fn src_pixel_from_target(&self, target_x: u32, target_y: u32) -> (u32, u32) {
        (
            (target_x as f64 / self.amount) as u32,
            (target_y as f64 / self.amount) as u32,
        )
    }
}

impl Transformation for Enlarge {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = ImageBuffer::new(
            (image.width() as f64 * self.amount) as u32,
            (image.height() as f64 * self.amount) as u32,
        );
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            let (src_x, src_y) = self.src_pixel_from_target(x, y);
            *pixel = *image.get_pixel(src_x, src_y);
        }
        *image = new_image;
        println!("After {}x{}", image.width(), (*image).height());
    }
}
