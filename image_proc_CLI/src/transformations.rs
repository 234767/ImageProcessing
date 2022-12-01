use crate::parsing::Args;
use image_proc::modifications::filters::basic::gpu::*;
use image_proc::modifications::prelude::*;
use image_proc::modifications::{IdTransform, Transformation};
use util::try_new_raleigh;

mod histogram;
mod util;

pub fn get_transformation(args: &Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--id" => Ok(Box::new(IdTransform {})),
        "--negative" => Ok(Box::new(Negative {})),
        "--brightness" => Ok(Box::new(Brightness::new(
            args.try_get_arg::<i32>("amount")?,
        ))),
        "--contrast" => Ok(Box::new(Contrast::new(args.try_get_arg::<f64>("amount")?))),
        "--hflip" => Ok(Box::new(HorizontalFlip {})),
        "--vflip" => Ok(Box::new(VerticalFlip {})),
        "--dflip" => Ok(Box::new(DiagonalFlip {})),
        "--shrink" => Ok(Box::new(util::try_new_shrink(args)?)),
        "--enlarge" => Ok(Box::new(util::try_new_enlarge(args)?)),
        "--median" => {
            let (width, height) = util::get_width_and_height(args)?;
            Ok(Box::new(MedianFilter::new(width, height)))
        }
        "--median-gpu" => {
            let (width, height) = util::get_width_and_height(args)?;
            match MedianFilterGPU::try_new(width, height) {
                Ok(filter) => Ok(Box::new(filter)),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    println!("Falling back to default implementation");
                    Ok(Box::new(MedianFilter::new(width, height)))
                }
            }
        }
        "--gmean" => {
            let (width, height) = util::get_width_and_height(args)?;
            Ok(Box::new(GeometricMeanFilter::new(width, height)))
        }
        "--gmean-gpu" => {
            let (width, height) = util::get_width_and_height(args)?;
            match GMeanFilterGPU::try_new(width, height) {
                Ok(filter) => Ok(Box::new(filter)),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    println!("Falling back to default implementation");
                    Ok(Box::new(GeometricMeanFilter::new(width, height)))
                }
            }
        }
        "--max-gpu" => {
            let (width, height) = util::get_width_and_height(args)?;
            match MaxFilterGPU::try_new(width, height) {
                Ok(filter) => Ok(Box::new(filter)),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("No default implementation to fallback to")
                }
            }
        }
        "--minimum" => {
            let (width, height) = util::get_width_and_height(args)?;
            Ok(Box::new(MinFilter::new(width, height)))
        }
        "--histogram" => Ok(Box::new(util::get_histogram_modifier(args)?)),
        "--lowpass" => Ok(Box::new(util::try_new_linear(args)?)),
        "--lowpass-gpu" => Ok(Box::new(util::try_new_linear_gpu(args)?)),
        "--hraleigh" => Ok(Box::new(try_new_raleigh(args)?)),
        "--uolis" => Ok(Box::new(UolisOperator {})),
        _ => Err(format!("Command {} undefined", args.command)),
    }
}
