use crate::modifications::morphological::dilation::Dilation;
use crate::modifications::morphological::erosion::Erosion;
use crate::modifications::morphological::{Mask, MorphologicalTransform};
use image::GrayImage;

pub struct Opening {
    mask: Mask,
}

impl Opening {
    pub fn new(mask: Mask) -> Self {
        Self { mask }
    }
}

impl_transform!(Opening);

impl MorphologicalTransform for Opening {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        Erosion::apply(&self.mask, image);
        Dilation::apply(&self.mask, image);
    }
}
