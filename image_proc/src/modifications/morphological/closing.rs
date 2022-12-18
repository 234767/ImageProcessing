use image::GrayImage;
use crate::modifications::morphological::erosion::Erosion;
use crate::modifications::morphological::dilation::Dilation;
use crate::modifications::morphological::{Mask, MorphologicalTransform};

pub struct Closing {
    pub erosion: Erosion,
    pub dilation: Dilation,
}

impl Closing {
    pub fn new(erosion_mask: Mask, dilation_mask: Mask) -> Self {
        Self {
            erosion: Erosion::new(erosion_mask),
            dilation: Dilation::new(dilation_mask),
        }
    }
}

impl_transform!(Closing);

impl MorphologicalTransform for Closing {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        self.dilation.apply_morph_operation(image);
        self.erosion.apply_morph_operation(image);
    }
}