use crate::parsing::Args;
use image_proc::modifications::*;
use crate::transformations::util::try_new_raleigh;

mod histogram;

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
            match gpu_optimized::MedianFilterGPU::try_new(width, height) {
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
            match gpu_optimized::GMeanFilterGPU::try_new(width, height) {
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
            match gpu_optimized::MaxFilterGPU::try_new(width, height) {
                Ok(filter) => Ok(Box::new(filter)),
                Err(e) => {
                    eprintln!("Error: {}", e);
                    panic!("No default implementation to fallback to")
                }
            }
        }
        "--minimum" => {
            let (width, height) = util::get_width_and_height(args)?;
            Ok(Box::new(MinimumFilter::new(width, height)))
        }
        "--histogram" => Ok(Box::new(util::get_histogram_modifier(args)?)),
        "--lowpass-gpu" => Ok(Box::new(gpu_optimized::LowPassFilterGPU::try_new()?)),
        "--hraleigh" => Ok(Box::new(try_new_raleigh(args)?)),
        "--uolis" => Ok(Box::new(Uolis{})),
        _ => Err(format!("Command {} undefined", args.command)),
    }
}

mod util {
    use crate::parsing::Args;
    use crate::transformations::histogram;
    use crate::transformations::histogram::HistogramConverter;
    use image_proc::modifications::{HRaleigh, Scale};
    use num::Integer;

    pub fn try_new_enlarge(args: &Args) -> Result<Scale, String> {
        let factor = args.try_get_arg("amount")?;
        Ok(Scale::new(factor, factor))
    }

    pub fn try_new_shrink(args: &Args) -> Result<Scale, String> {
        // invert the factor - shrink x2 = scale x0.5
        let factor = 1f64 / args.try_get_arg::<f64>("amount")?;
        Ok(Scale::new(factor, factor))
    }

    pub fn get_width_and_height(args: &Args) -> Result<(u32, u32), String> {
        let mut width: u32 = args.try_get_arg("-w")?;
        if width.is_even() {
            width += 1
        }
        let mut height: u32 = args.try_get_arg("-h")?;
        if height.is_even() {
            height += 1
        }
        Ok((width, height))
    }

    pub fn get_histogram_modifier(args: &Args) -> Result<HistogramConverter, String> {
        match args.args.get("-c") {
            Some(channel_arg) => {
                let channel = match channel_arg.as_str() {
                    "r" => histogram::HistogramChannelOptions::R,
                    "g" => histogram::HistogramChannelOptions::G,
                    "b" => histogram::HistogramChannelOptions::B,
                    "all" => histogram::HistogramChannelOptions::All,
                    _ => panic!("Invalid option"),
                };
                Ok(HistogramConverter::new(channel))
            }
            _ => Err(String::from("Missing -c argument")),
        }
    }

    pub fn try_new_raleigh(args: &Args) -> Result<HRaleigh, String> {
        let gmin: u8 = args.try_get_arg("-gmin")?;
        let gmax: u8 = args.try_get_arg("-gmax")?;
        Ok(HRaleigh::new(gmin, gmax))
    }
}
