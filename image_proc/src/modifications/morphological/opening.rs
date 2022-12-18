use image::GrayImage;
use crate::modifications::morphological::dilation::Dilation;
use crate::modifications::morphological::erosion::Erosion;
use crate::modifications::morphological::{Mask, MorphologicalTransform};

pub struct Opening {
    pub dilation: Dilation,
    pub erosion: Erosion,
}

impl Opening {
    pub fn new(dilation_mask: Mask, erosion_mask: Mask) -> Self {
        Self {
            dilation: Dilation::new(dilation_mask),
            erosion: Erosion::new(erosion_mask),
        }
    }
}

impl_transform!(Opening);

impl MorphologicalTransform for Opening {
    fn apply_morph_operation(&self, image: &mut GrayImage) {
        self.erosion.apply_morph_operation(image);
        self.dilation.apply_morph_operation(image);
    }
}
