use crate::parsing::Args;
use crate::transformations::histogram;
use crate::transformations::histogram::HistogramConverter;
use image_proc::modifications::filters::linear::optimized::LinearFilterGPU;
use image_proc::modifications::geometric::Scale;
use image_proc::modifications::prelude::*;
use image_proc::modifications::segmentation::RegionGrowing;
use num::Integer;
use std::num::{ParseFloatError, ParseIntError};

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

pub fn try_new_raleigh(args: &Args) -> Result<RayleighModification, String> {
    let gmin: u8 = args.try_get_arg("-gmin")?;
    let gmax: u8 = args.try_get_arg("-gmax")?;
    Ok(RayleighModification::new(gmin, gmax))
}

fn try_parse_mask(args: &Args) -> Result<[f64; 9], String> {
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
            debug_assert_eq!(9, mask.len());
            Ok(mask.try_into().unwrap())
        }
        None => Err(String::from("Missing -mask argument")),
    }
}

pub fn try_parse_kernel(args: &Args) -> Result<Vec<i32>, String> {
    match args.args.get("-kernel") {
        Some(mask_string) => {
            let mask: Vec<Result<i32, ParseIntError>> =
                mask_string.split(";").map(|s| s.parse()).collect();
            if mask.len() != 9 {
                return Err(format!("Expected kernel of length of 9, got {}", mask.len()));
            }
            if let Some(Err(e)) = mask.iter().find(|x| x.is_err()) {
                return Err(e.to_string());
            }
            let mask: Vec<i32> = mask.into_iter().map(|x| x.unwrap()).collect();
            debug_assert_eq!(9, mask.len());
            Ok(mask)
        }
        None => Err(String::from("Missing -kernel argument")),
    }
}

pub fn try_parse_hmt_kernel(args: &Args) -> Result<(Vec<u8>, Vec<u8>), String> {
    let kernel = try_parse_kernel(args)?;

    let hit: Vec<u8> = kernel
        .clone()
        .into_iter()
        .map(|x| if x > 0 { 1 } else { 0 })
        .collect();
    let miss: Vec<u8> = kernel
        .into_iter()
        .map(|x| if x < 0 { 1 } else { 0 })
        .collect();

    debug_assert_eq!(hit.len(), miss.len());

    Ok((hit, miss))
}

fn try_parse_mask_scale(args: &Args) -> Option<Result<f64, String>> {
    const MASK_SEPARATOR: &str = "/";
    match args.args.get("-mask-scale") {
        Some(fraction) if fraction.contains(MASK_SEPARATOR) => {
            let nums: Vec<_> = fraction
                .split(MASK_SEPARATOR)
                .map(|s| s.parse::<f64>())
                .collect();
            if nums.len() != 2 {
                return Some(Err(format!(
                    "Mask scale in fraction form expected to have 2 parts, got {}",
                    nums.len()
                )));
            }
            if let Some(Err(e)) = nums.iter().find(|x| x.is_err()) {
                return Some(Err(e.to_string()));
            }
            let nums: Vec<f64> = nums.into_iter().map(|x| x.unwrap()).collect();
            debug_assert_eq!(2, nums.len());
            Some(Ok(nums[0] / nums[1]))
        }
        Some(scale) => match scale.parse::<f64>() {
            Ok(scale) => Some(Ok(scale)),
            Err(e) => Some(Err(e.to_string())),
        },
        _ => None,
    }
}

pub fn try_new_linear(args: &Args) -> Result<LinearFilter, String> {
    let mask = try_parse_mask(args)?;
    match try_parse_mask_scale(args) {
        Some(Ok(scale)) => Ok(LinearFilter::from_flat_mask(
            mask.try_into().unwrap(),
            Some(scale),
        )),
        Some(Err(e)) => {
            eprintln!(
                "Error while parsing -mask-scale argument: {}",
                e.to_string()
            );
            Ok(LinearFilter::from_flat_mask(mask.try_into().unwrap(), None))
        }
        _ => Ok(LinearFilter::from_flat_mask(mask.try_into().unwrap(), None)),
    }
}

pub fn try_new_linear_gpu(args: &Args) -> Result<LinearFilterGPU, String> {
    let mask = try_parse_mask(args)?;
    match try_parse_mask_scale(args) {
        Some(Ok(scale)) => Ok(LinearFilterGPU::try_new(
            mask.try_into().unwrap(),
            Some(scale),
        )?),
        Some(Err(e)) => {
            eprintln!(
                "Error while parsing -mask-scale argument: {}",
                e.to_string()
            );
            Ok(LinearFilterGPU::try_new(mask.try_into().unwrap(), None)?)
        }
        _ => Ok(LinearFilterGPU::try_new(mask.try_into().unwrap(), None)?),
    }
}

pub fn try_new_region_grow(args: &Args) -> Result<RegionGrowing, String> {
    let seed_x: u32 = args.try_get_arg("-x")?;
    let seed_y: u32 = args.try_get_arg("-y")?;
    let tolerance: u8 = args.try_get_arg("-tolerance")?;
    Ok(RegionGrowing::new(seed_x, seed_y, tolerance))
}
