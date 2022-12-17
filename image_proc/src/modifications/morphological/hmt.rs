use super::{Mask, MorphologicalTransform};
use crate::modifications::morphological::FOREGROUND_PIXEL;
use image::{GrayImage, ImageBuffer};

pub struct HitOrMissTransform {
    hit_mask: Mask,
    miss_mask: Mask,
}

impl HitOrMissTransform {
    pub fn new(hit_mask: Mask, miss_mask: Mask) -> Self {
        Self {
            hit_mask,
            miss_mask,
        }
    }
}

impl_transform!(HitOrMissTransform);

impl MorphologicalTransform for HitOrMissTransform {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        let mut new_image: GrayImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, _) in image.enumerate_pixels() {

            let img_mask = &Mask::from_image(image, x, y);
            let is_hit = self.hit_mask == (&self.hit_mask & img_mask);
            let is_miss = self.miss_mask == (&self.miss_mask & &(!img_mask));

            if is_hit && is_miss {
                new_image.put_pixel(x, y, FOREGROUND_PIXEL)
            }
        }
        *image = new_image;
    }
}
