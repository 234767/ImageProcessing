use crate::modifications::Transformation;
use image::imageops::grayscale;
use image::{DynamicImage, GrayImage, Luma, RgbImage};

const LUMA_THRESHOLD: u8 = 128;
const FOREGROUND_PIXEL: Luma<u8> = Luma([255]);
#[allow(dead_code)]
const BACKGROUND_PIXEL: Luma<u8> = Luma([0]);

pub trait MorphologicalTransform: Transformation {
    fn apply_morph_operation(&self, image: &mut GrayImage);

    fn apply(&self, image: &mut RgbImage) {
        let mut grayscale = grayscale(image);
        self.apply_morph_operation(&mut grayscale);
        *image = DynamicImage::from(grayscale).to_rgb8();
    }
}

macro_rules! impl_transform {
    ($name: ident) => {
        impl crate::modifications::Transformation for $name {
            fn apply(&self, image: &mut image::RgbImage) {
                MorphologicalTransform::apply(self, image)
            }
        }
    };
}

fn is_foreground(pixel: &Luma<u8>) -> bool {
    let Luma([luma]) = *pixel;
    luma > LUMA_THRESHOLD
}

pub mod mask;
pub use mask::Mask;

pub mod dilation;
pub mod erosion;
pub mod opening;
pub mod closing;
pub mod hmt;