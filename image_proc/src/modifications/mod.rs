use image::RgbImage;

mod elementary;
mod filters;
mod geometric;

pub use elementary::*;
pub use filters::*;
pub use geometric::*;

pub trait Transformation {
    fn apply(&self, image: &mut RgbImage);
}

/// Does nothing with the image
pub struct IdTransform {}

impl Transformation for IdTransform {
    fn apply(&self, _image: &mut RgbImage) {}
}


