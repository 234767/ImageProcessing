use super::{hmt::HitOrMissTransform, Mask, MorphologicalTransform, FOREGROUND_PIXEL};
use image::GrayImage;

pub struct ConvexHull;

static STRUCTURAL_ELEMENTS: [(Mask, Mask); 4] = [
    (
        Mask::from_raw_data(0b001001001),
        Mask::from_raw_data(0b000010000),
    ),
    (
        Mask::from_raw_data(0b000000111),
        Mask::from_raw_data(0b000010000),
    ),
    (
        Mask::from_raw_data(0b100100100),
        Mask::from_raw_data(0b000010000),
    ),
    (
        Mask::from_raw_data(0b111000000),
        Mask::from_raw_data(0b000010000),
    ),
];

impl_transform!(ConvexHull);

fn saturate_with_transform(image: &mut GrayImage, transform: &HitOrMissTransform) {
    loop {
        let transform_result: Vec<_> = transform.get_white_pixels(image).collect();
        if transform_result.len() == 0 {
            return;
        }
        for (x, y) in transform_result {
            image.put_pixel(x, y, FOREGROUND_PIXEL);
        }
    }
}

impl MorphologicalTransform for ConvexHull {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        for (hit, miss) in &STRUCTURAL_ELEMENTS {
            let transform = HitOrMissTransform::new(hit.clone(), miss.clone());
            saturate_with_transform(image, &transform);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::saturate_with_transform;
    use crate::modifications::morphological::hmt::HitOrMissTransform;
    use crate::modifications::morphological::{is_foreground, Mask, FOREGROUND_PIXEL};
    use image::{GrayImage, ImageBuffer};

    #[test]
    fn saturate_with_transform_works() {
        let mut image: GrayImage = ImageBuffer::new(3, 3);
        image.put_pixel(0, 0, FOREGROUND_PIXEL);
        image.put_pixel(1, 0, FOREGROUND_PIXEL);
        image.put_pixel(2, 0, FOREGROUND_PIXEL);
        image.put_pixel(0, 1, FOREGROUND_PIXEL);
        image.put_pixel(2, 1, FOREGROUND_PIXEL);

        let transformation = HitOrMissTransform::new(
            Mask::from_raw_data(0b000000111),
            Mask::from_raw_data(0b000010000),
        );

        saturate_with_transform(&mut image, &transformation);

        assert!(is_foreground(image.get_pixel(1, 1)));
        assert!(is_foreground(image.get_pixel(1, 2)));
    }
}
