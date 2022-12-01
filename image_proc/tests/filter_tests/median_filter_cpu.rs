use crate::*;
use image_proc::modifications::filters::basic::median_filter::MedianFilter;

fn test_median_filter(width: u32, height: u32, channel: usize) {
    let mut image = sample_image();

    let filter = MedianFilter::new(width, height);

    let values: Vec<u8> = {
        let mut values: Vec<u8> = (0..width * height).map(|i| (i * i) as u8).collect();
        values.sort();
        values
    };
    assert_eq!(width * height, values.len() as u32);
    let median = values[values.len() / 2];

    let mut iter = values.iter();
    for xi in 1..(1 + width) {
        for yi in 1..(1 + height) {
            let Rgb(pixel) = image.get_pixel_mut(xi, yi);
            pixel[channel] = *iter.next().unwrap();
        }
    }

    filter.apply(&mut image);
    let (target_x, target_y) = ((1 + width) / 2, (1 + height) / 2);
    let Rgb(target_pixel) = image.get_pixel(target_x, target_y);
    assert_eq!(median, target_pixel[channel]);
}

invoke_test! { test_median_filter {
    median_3x3_red  (3,3,0),
    median_3x3_green (3,3,1),
    median_3x3_blue (3,3,2),
    median_5x5_red (5,5,0),
    median_5x5_green (5,5,1),
    median_5x5_blue (5,5,2),
    median_7x7_red (7,7,0),
    median_7x7_green (7,7,1),
    median_7x7_blue (7,7,2),
    median_9x9_red (9,9,0),
    median_9x9_green (9,9,1),
    median_9x9_blue (9,9,2)
}}
