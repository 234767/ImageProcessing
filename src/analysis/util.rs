use image::{Rgb, RgbImage};
use image::buffer::Pixels;

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
    map_and_reduce(original, modified, function, |a,b| a + b, Rgb([0,0,0]))
}

pub(crate) fn map_and_reduce<F1, F2>(original: &RgbImage, modified: &RgbImage, function: F1, folder: F2, initial_state: Rgb<i128>) -> Rgb<i128>
    where F1: Fn(u8, u8) -> i128,
          F2: Fn(i128, i128) -> i128
{
    let total = initial_state;
    let iterator = DoubleImageIterator::new(original, modified);
    for (old_pixel, new_pixel) in iterator {
        for channel in 1..3 {
            let value = function(old_pixel[channel], new_pixel[channel]);
            total[channel] = folder(total[channel], value);
        }
    };
    total
}

pub struct DoubleImageIterator<'a> {
    old_pixels: Pixels<'a, Rgb<u8>>,
    new_pixels: Pixels<'a, Rgb<u8>>,
}

impl<'a> DoubleImageIterator<'a> {
    pub fn new(original: &'a RgbImage, modified: &'a RgbImage) -> Self {
        Self {
            old_pixels: original.pixels(),
            new_pixels: modified.pixels(),
        }
    }
}

impl<'a> Iterator for DoubleImageIterator<'a> {
    type Item = (&'a Rgb<u8>, &'a Rgb<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        Some((self.old_pixels.next()?, self.new_pixels.next()?))
    }
}
