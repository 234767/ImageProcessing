use super::{Mask, MorphologicalTransform, FOREGROUND_PIXEL};
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

    pub(crate) fn get_matching_pixels<'img, 's, 'out>(
        &'s self,
        image: &'img GrayImage,
    ) -> impl Iterator<Item = (u32, u32)> + 'out
    where
        'img: 'out,
        's: 'out,
    {
        image
            .enumerate_pixels()
            .map(|(x, y, _)| (x, y))
            .filter(|(x, y)| {
                let img_mask = &Mask::from_image(image, *x, *y);
                let is_hit = self.hit_mask == (&self.hit_mask & img_mask);
                let is_miss = self.miss_mask == (&self.miss_mask & &(!img_mask));

                is_hit && is_miss
            })
    }
}

impl_transform!(HitOrMissTransform);

impl MorphologicalTransform for HitOrMissTransform {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        let mut new_image: GrayImage = ImageBuffer::new(image.width(), image.height());
        for (x, y) in self.get_matching_pixels(image) {
            new_image.put_pixel(x, y, FOREGROUND_PIXEL);
        }
        *image = new_image;
    }
}
