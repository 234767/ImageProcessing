use image::RgbImage;

pub mod elementary;
pub mod filters;
pub mod geometric;
pub mod histogram_modifications;

pub mod prelude {
    pub use super::elementary::*;
    pub use super::filters::*;
    pub use super::geometric::*;
    pub use super::histogram_modifications::*;
}

pub trait Transformation {
    fn apply(&self, image: &mut RgbImage);
}

/// Does nothing with the image
pub struct IdTransform;

impl Transformation for IdTransform {
    fn apply(&self, _image: &mut RgbImage) {}
}
