use crate::parsing::Args;
use image::RgbImage;

mod elementary;
mod filters;
mod geometric;

use elementary::*;
use filters::*;
use geometric::*;

pub trait Transformation {
    fn apply(&self, image: &mut RgbImage);
}

pub fn get_transformation(args: &Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--negative" => Ok(Box::new(Negative {})),
        "--brightness" => Ok(Box::new(Brightness::try_new(args)?)),
        "--contrast" => Ok(Box::new(Contrast::try_new(args)?)),
        "--hflip" => Ok(Box::new(HorizontalFlip {})),
        "--vflip" => Ok(Box::new(VerticalFlip {})),
        "--dflip" => Ok(Box::new(DiagonalFlip {})),
        "--shrink" => Ok(Box::new(Scale::try_new_shrink(args)?)),
        "--enlarge" => Ok(Box::new(Scale::try_new_enlarge(args)?)),
        "--median" => Ok(Box::new(MedianFilter::try_new(args)?)),
        "--median-gpu" => match gpu_optimized::MedianFilterGPU::try_new(args) {
            Ok(filter) => Ok(Box::new(filter)),
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Falling back to default implementation");
                Ok(Box::new(MedianFilter::try_new(args)?))
            }
        },
        "--gmean" => Ok(Box::new(GeometricMeanFilter::try_new(args)?)),
        "--gmean-gpu" => match gpu_optimized::GMeanFilterGPU::try_new(args) {
            Ok(filter) => Ok(Box::new(filter)),
            Err(e) => {
                eprintln!("Error: {}", e);
                println!("Falling back to default implementation");
                Ok(Box::new(MedianFilter::try_new(args)?))
            }
        },
        "--max-gpu" => match gpu_optimized::MaxFilterGPU::try_new(args) {
            Ok(filter) => Ok(Box::new(filter)),
            Err(e) => {
                panic!("Error: {}", e);
            }
        },
        _ => Err(format!("Command {} undefined", args.command)),
    }
}