use image::RgbImage;
use crate::modifications::Transformation;

///(H3) Raleigh final probability density function (--hraleigh).
pub struct HRaleigh {
    width: u32,
    height: u32,
}

impl Transformation for HRaleigh {
    fn apply(&self, image: &mut RgbImage) {
        todo!()
    }
}
