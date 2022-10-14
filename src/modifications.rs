use crate::parsing::Args as Args;
use num;
use image::{Pixel, RgbImage};

pub trait Transformation {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage;
}

pub fn get_transformation(args: &Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--negative" => Ok(Box::new(Negative {})),
        "--brightness" => Ok(Box::new(Brightness::try_new(args)?)),
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

pub struct Brightness {
    amount: i32
}

impl Brightness {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let amount = args.args.get("-amount");
        match amount {
            Some(amount) => match amount.parse::<i32>() {
                Ok(amount) => Ok(Self { amount }),
                Err(_) => Err(format!("Amount {} is not an integer", amount)),
            },
            None => Err(String::from("Missing -amount argument")),
        }
    }
}

impl Transformation for Brightness {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|c| num::clamp(c as i32 + self.amount,0,u8::MAX as i32) as u8);
        }
        image
    }
}
