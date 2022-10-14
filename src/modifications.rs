use crate::parsing::Args;
use image::RgbImage;

mod elementary;
mod geometric;

use elementary::*;
use geometric::*;

pub trait Transformation {
    fn apply<'a>(&self, image: &'a mut RgbImage) -> &'a mut RgbImage;
}

pub fn get_transformation(args: &Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--negative" => Ok(Box::new(Negative {})),
        "--brightness" => Ok(Box::new(Brightness::try_new(args)?)),
        "--contrast" => Ok(Box::new(Contrast::try_new(args)?)),
        "--hflip" => Ok(Box::new(HorizontalFlip {})),
        "--vflip" => Ok(Box::new(VerticalFlip {})),
        _ => Err(format!("Command {} undefined", args.command)),
    }
}
