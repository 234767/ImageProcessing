use crate::modifications::Transformation;
use image::{ImageBuffer, RgbImage};

pub struct RegionGrowing();

impl Transformation for RegionGrowing{
    fn apply(&self, image: &mut RgbImage){
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (_x, _y, _pixel) in new_image.enumerate_pixels_mut() {
            todo!()
        }
        *image = new_image;

    }
}
