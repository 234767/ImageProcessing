use crate::modifications::Transformation;
use image::RgbImage;

//(G1) Horizontal flip (--hflip)
pub struct HorizontalFlip;

impl HorizontalFlip {
    fn apply(image: &mut RgbImage) {
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

impl Transformation for HorizontalFlip {
    fn apply(&self, image: &mut RgbImage) {
        HorizontalFlip::apply(image);
    }
}

//(G2) Vertical flip (--vflip)
pub struct VerticalFlip;

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
        VerticalFlip::apply(image);
    }
}

//(G3) Diagonal flip (--dflip)
pub struct DiagonalFlip;

impl Transformation for DiagonalFlip {
    fn apply(&self, image: &mut RgbImage) {
        HorizontalFlip::apply(image);
        VerticalFlip::apply(image);
    }
}
