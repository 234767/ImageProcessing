use super::{is_foreground, Mask, MorphologicalTransform, FOREGROUND_PIXEL};
use image::{GrayImage, ImageBuffer};

pub struct Erosion {
    mask: Mask,
}

impl Erosion {
    pub fn new(mask: Mask) -> Self {
        Self { mask }
    }

    pub(crate) fn apply(mask: &Mask, image: &mut GrayImage) {
        let mut new_image: GrayImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, pixel) in image.enumerate_pixels() {
            if !is_foreground(pixel) {
                continue;
            }
            let new_mask = mask & &Mask::from_image(&image, x, y);
            if &new_mask == mask {
                new_image.put_pixel(x, y, FOREGROUND_PIXEL);
            }
        }
        *image = new_image;
    }
}

impl_transform!(Erosion);

impl MorphologicalTransform for Erosion {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        Self::apply(&self.mask, image);
    }
}
