use crate::parsing::Args;
use crate::transformations::histogram;
use crate::transformations::histogram::HistogramConverter;
use image_proc::modifications::gpu_optimized::LowPassFilterGPU;
use image_proc::modifications::{HRaleigh, LowPassFilter, Scale};
use num::Integer;
use std::num::ParseFloatError;

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

pub fn try_new_lowpass(args: &Args) -> Result<LowPassFilter, String> {
    match args.args.get("-mask") {
        Some(mask_string) => {
            let mask: Vec<Result<f64, ParseFloatError>> =
                mask_string.split(";").map(|s| s.parse()).collect();
            if mask.len() != 9 {
                return Err(format!("Expected mask length of 9, got {}", mask.len()));
            }
            if let Some(Err(e)) = mask.iter().find(|x| x.is_err()) {
                return Err(e.to_string());
            }
            let mask: Vec<f64> = mask.into_iter().map(|x| x.unwrap()).collect();
            match args.args.get("-mask-scale").map(|s| s.parse::<f64>()) {
                Some(Ok(scale)) => Ok(LowPassFilter::from_flat_mask(
                    mask.try_into().unwrap(),
                    Some(scale),
                )),
                Some(Err(e)) => {
                    eprintln!(
                        "Error while parsing -mask-scale argument: {}",
                        e.to_string()
                    );
                    Ok(LowPassFilter::from_flat_mask(
                        mask.try_into().unwrap(),
                        None,
                    ))
                }
                _ => Ok(LowPassFilter::from_flat_mask(
                    mask.try_into().unwrap(),
                    None,
                )),
            }
        }
        None => Err(String::from("Missing -mask argument")),
    }
}

pub fn try_new_lowpass_gpu(args: &Args) -> Result<LowPassFilterGPU, String> {
    match args.args.get("-mask") {
        Some(mask_string) => {
            let mask: Vec<Result<f64, ParseFloatError>> =
                mask_string.split(";").map(|s| s.parse()).collect();
            if mask.len() != 9 {
                return Err(format!("Expected mask length of 9, got {}", mask.len()));
            }
            if let Some(Err(e)) = mask.iter().find(|x| x.is_err()) {
                return Err(e.to_string());
            }
            let mask: Vec<f64> = mask.into_iter().map(|x| x.unwrap()).collect();
            match args.args.get("-mask-scale").map(|s| s.parse::<f64>()) {
                Some(Ok(scale)) => Ok(LowPassFilterGPU::try_new(
                    mask.try_into().unwrap(),
                    Some(scale),
                )?),
                Some(Err(e)) => {
                    eprintln!(
                        "Error while parsing -mask-scale argument: {}",
                        e.to_string()
                    );
                    Ok(LowPassFilterGPU::try_new(mask.try_into().unwrap(), None)?)
                }
                _ => Ok(LowPassFilterGPU::try_new(mask.try_into().unwrap(), None)?),
            }
        }
        None => Err(String::from("Missing -mask argument")),
    }
}
