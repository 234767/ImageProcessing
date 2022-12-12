use super::mask::Mask;
use super::MorphologicalTransform;
use crate::modifications::morphological::is_foreground;
use image::{GrayImage, ImageBuffer};

pub struct Dilation {
    mask: Mask,
}

impl Dilation {
    pub fn new(mask: Mask) -> Self {
        Self { mask }
    }
}

impl_transform!(Dilation);

impl MorphologicalTransform for Dilation {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        let mut new_image: GrayImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, pixel) in image.enumerate_pixels() {
            if !is_foreground(pixel) {
                continue;
            }
            let new_image_mask = Mask::from_image(&new_image, x, y);
            let mask =
                self.mask | new_image_mask;
            mask.write_to_image(&mut new_image, x, y);
        }
        *image = new_image;
    }
}
