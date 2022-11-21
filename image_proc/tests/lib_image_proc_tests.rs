use image::{Rgb, RgbImage};
use image_proc::modifications::Transformation;

macro_rules! invoke_test {
    ($func:ident {$($name:ident ($($arg:expr),*)),*}) => {
        $(
            #[test]
            fn $name () {
                $func($($arg),*);
            }
        )*
    };
    // allow for trailing comma:
    ($func:ident {$($name:ident ($($arg:expr),*),)*}) => {
        $(
            #[test]
            fn $name () {
                $func($($arg),*);
            }
        )*
    }
}

fn sample_image() -> RgbImage {
    // image size 10x10
    let width = 10;
    let height = 10;
    let buffer: Vec<u8> = (0..(width * height * 3)).map(|c| c as u8).collect();
    let image: RgbImage = RgbImage::from_vec(width, height, buffer).unwrap();
    image
}

#[test]
fn sample_image_setup_correctly() {
    let _ = sample_image();
}

mod filter_tests;
