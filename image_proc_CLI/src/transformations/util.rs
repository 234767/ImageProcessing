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
