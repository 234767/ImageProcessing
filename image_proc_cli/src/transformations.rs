use crate::parsing::Args;
use image_proc::modifications::*;

pub fn get_transformation(args: &Args) -> Result<Box<dyn Transformation>, String> {
    match args.command.as_str() {
        "--negative" => Ok(Box::new(Negative {})),
        "--brightness" => Ok(Box::new(Brightness::try_new(args)?)),
        "--contrast" => Ok(Box::new(Contrast::try_new(args)?)),
        "--hflip" => Ok(Box::new(HorizontalFlip {})),
        "--vflip" => Ok(Box::new(VerticalFlip {})),
        "--dflip" => Ok(Box::new(DiagonalFlip {})),
        "--shrink" => Ok(Box::new(util::try_new_shrink(args)?)),
        "--enlarge" => Ok(Box::new(util::try_new_enlarge(args)?)),
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

mod util {
    use crate::parsing::Args;
    use image_proc::modifications::{Brightness, Contrast, Scale};

    fn get_scale(args: &Args) -> Result<f64, String> {
        let amount = args.try_get_arg("amount")?;
        if amount < 0.0 {
            Err(format!("Number {} is not a positive number!", amount))
        } else {
            Ok(amount)
        }
    }

    pub fn try_new_enlarge(args: &Args) -> Result<Scale, String> {
        let factor = get_scale(args)?;
        Ok(Self {
            factor_x: factor,
            factor_y: factor,
        })
    }

    pub fn try_new_shrink(args: &Args) -> Result<Scale, String> {
        // invert the factor - shrink x2 = scale x0.5
        let factor = 1f64 / get_scale(args)?;
        Ok(Self {
            factor_x: factor,
            factor_y: factor,
        })
    }

    pub fn try_new_brightness(args: &Args) -> Result<Brightness, String> {
        let amount: i32 = args.try_get_arg("amount")?;
        if amount < 0 {
            Err(format!("Number {} is not a positive integer!", amount))
        } else {
            Ok(Brightness::new(amount))
        }
    }

    pub fn try_new_contrast(args: &Args) -> Result<Contrast, String> {
        let factor: f64 = args.try_get_arg("amount")?;
        if factor < 0.0 {
            Err(format!("Number {} is not a positive integer!", factor))
        } else {
            Ok(Contrast::new(factor))
        }
    }
}
