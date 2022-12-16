use std::cmp::max;
use image::{GrayImage, Luma};
use crate::modifications::morphological::hmt::HitOrMissTransform;
use super::{MorphologicalTransform, Mask};

pub struct ConvexHull;

static STRUCTURAL_ELEMENTS: [(Mask,Mask);4] = [
    (Mask::from_raw_bits(0b001001001),Mask::from_raw_bits(0b110100110)),
    (Mask::from_raw_bits(0b000000111),Mask::from_raw_bits(0b111101000)),
    (Mask::from_raw_bits(0b100100100),Mask::from_raw_bits(0b011001011)),
    (Mask::from_raw_bits(0b111000000),Mask::from_raw_bits(0b000101111)),
];

impl_transform!(ConvexHull);

fn saturate_with_transform(image: &mut GrayImage, transform: &impl MorphologicalTransform){
    loop {
        let mut new_image = image.clone();
        transform.apply_morph_operation(&mut new_image);
        if new_image == *image {
            return;
        }
        image_union(image, &new_image);
    }
}

fn image_union(image: &mut GrayImage, new_image: &GrayImage) {
    for (x,y,pixel) in image.enumerate_pixels_mut() {
        let new_pixel = new_image.get_pixel(x,y);
        *pixel = Luma([max(pixel[0], new_pixel[0])]);
    }
}

impl MorphologicalTransform for ConvexHull {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        for (hit,miss) in &STRUCTURAL_ELEMENTS {
            let transform = HitOrMissTransform::new(hit.clone(), miss.clone());
            saturate_with_transform(image, &transform);
        }
    }
}