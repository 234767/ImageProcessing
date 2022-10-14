use crate::parsing::Args;
use image::{Pixel, RgbImage};
use num;

pub trait Transformation {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage;
}

pub fn get_transformation(args: &Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--negative" => Ok(Box::new(Negative {})),
        "--brightness" => Ok(Box::new(Brightness::try_new(args)?)),
        "--contrast" => Ok(Box::new(Contrast::try_new(args)?)),
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
    amount: i32,
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
            *pixel = pixel.map(|c| num::clamp(c as i32 + self.amount, 0, u8::MAX as i32) as u8);
        }
        image
    }
}

struct Contrast {
    factor: f64,
}

impl Contrast {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let amount = args.args.get("-amount");
        match amount {
            Some(amount) => match amount.parse::<f64>() {
                Ok(factor) if factor >= 0.0 => Ok(Self { factor }),
                _ => Err(format!("Amount {} is not a positive number", amount)),
            },
            None => Err(String::from("Missing -amount argument")),
        }
    }
}

impl Transformation for Contrast {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage {
        for (_x, _y, pixel) in image.enumerate_pixels_mut() {
            *pixel = pixel.map(|c| num::clamp((c as f64 - 128f64) * self.factor + 128f64, 0f64, u8::MAX as f64) as u8);
        }
        image
    }
}
