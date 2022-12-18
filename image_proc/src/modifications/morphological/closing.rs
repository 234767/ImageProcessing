use crate::modifications::morphological::dilation::Dilation;
use crate::modifications::morphological::erosion::Erosion;
use crate::modifications::morphological::{Mask, MorphologicalTransform};
use image::GrayImage;

pub struct Closing {
    mask: Mask,
}

impl Closing {
    pub fn new(mask: Mask) -> Self {
        Self { mask }
    }
}

impl_transform!(Closing);

impl MorphologicalTransform for Closing {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        Dilation::apply(&self.mask, image);
        Erosion::applly(&self.mask, image);
    }
}
