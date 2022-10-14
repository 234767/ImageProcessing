use image::{Pixel, RgbImage};

pub trait Transformation {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage;
}

pub fn get_transformation(args: &crate::parsing::Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--negative" => Ok(Box::new(Negative {})),
        _ => Err(format!("Command {} undefined", args.command)),
    }
}

pub struct Negative {}

impl Transformation for Negative {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|x| 255 - x);
        }
        image
    }
}
