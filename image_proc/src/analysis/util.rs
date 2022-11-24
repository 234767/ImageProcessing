use image::{Rgb, RgbImage};

/// For each channel of RGB maps a given function, and sums the results
///
/// # Arguments
///
/// * `original`: original image
/// * `modified`: modified image
/// * `function`: function to map the different brightness values
///
/// returns: Rgb<i128>
/// Values for RGB channels respectively
pub(crate) fn map_and_sum<F>(original: &RgbImage, modified: &RgbImage, function: F) -> Rgb<i128>
where
    F: Fn(u8, u8) -> i128,
{
    map_and_reduce(original, modified, function, |a, b| a + b, Rgb([0, 0, 0]))
}

pub(crate) fn map_and_reduce<F1, F2>(
    original: &RgbImage,
    modified: &RgbImage,
    function: F1,
    folder: F2,
    initial_state: Rgb<i128>,
) -> Rgb<i128>
where
    F1: Fn(u8, u8) -> i128,
    F2: Fn(i128, i128) -> i128,
{
    let mut total = initial_state;
    for (old_pixel, new_pixel) in original.pixels().zip(modified.pixels()) {
        for channel in 1..3 {
            let value = function(old_pixel[channel], new_pixel[channel]);
            total[channel] = folder(total[channel], value);
        }
    }
    total
}
