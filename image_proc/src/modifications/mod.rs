use image::{Pixel, RgbImage};
use std::ops::Deref;
pub mod elementary;
pub mod filters;
pub mod geometric;
pub mod histogram_modifications;
pub mod morphological;
pub mod segmentation;

pub mod prelude {
    pub use super::elementary::*;
    pub use super::filters::basic::{GeometricMeanFilter, MaxFilter, MedianFilter, MinFilter};
    pub use super::filters::linear::LinearFilter;
    pub use super::filters::nonlinear::UolisOperator;
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

fn is_edge<P, Container>(image: &image::ImageBuffer<P, Container>, x: u32, y: u32) -> bool
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    0 == x || x == image.width() - 1 || 0 == y || y == image.height() - 1
}
