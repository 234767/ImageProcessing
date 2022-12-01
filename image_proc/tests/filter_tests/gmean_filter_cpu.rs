use crate::*;
use image_proc::modifications::filters::basic::gmean_filter::GeometricMeanFilter;

fn test_gmean_filter(width: u32, height: u32, channel: usize) {
    let mut image = sample_image();

    let filter = GeometricMeanFilter::new(width, height);

    let values: Vec<u8> = (0..width * height).map(|i| (i * i) as u8).collect();

    assert_eq!(width * height, values.len() as u32);

    let geometric_mean: u8 =
        (values.iter().map(|p| *p as f64).product::<f64>() / values.len() as f64) as u8;

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
    assert_eq!(geometric_mean, target_pixel[channel]);
}

invoke_test! { test_gmean_filter {
    gmean_3x3_red  (3,3,0),
    gmean_3x3_green (3,3,1),
    gmean_3x3_blue (3,3,2),
    gmean_5x5_red (5,5,0),
    gmean_5x5_green (5,5,1),
    gmean_5x5_blue (5,5,2),
    gmean_7x7_red (7,7,0),
    gmean_7x7_green (7,7,1),
    gmean_7x7_blue (7,7,2),
    gmean_9x9_red (9,9,0),
    gmean_9x9_green (9,9,1),
    gmean_9x9_blue (9,9,2)
}}
