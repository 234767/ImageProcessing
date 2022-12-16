use image::Luma;

mod dilation_tests;
mod erosion_tests;
mod hmt_tests;

fn is_foreground(pixel: &Luma<u8>) -> bool {
    let Luma([luma]) = *pixel;
    luma > 128u8
}
