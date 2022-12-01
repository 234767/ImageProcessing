use crate::sample_image;
use image::Rgb;
use image_proc::modifications::filters::basic::gpu::max_filter::MaxFilterGPU;
use image_proc::modifications::Transformation;

fn test_max_filter(width: u32, height: u32, channel: usize) {
    let mut image = sample_image();

    let filter = MaxFilterGPU::try_new(width, height).unwrap();

    let values: Vec<u8> = (0..width * height).map(|i| (i * i) as u8).collect();

    assert_eq!(width * height, values.len() as u32);

    let max_value: u8 = *values.iter().max().unwrap();

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
    assert_eq!(max_value, target_pixel[channel]);
}

invoke_test! {test_max_filter{
    max_3x3_red  (3,3,0),
    max_3x3_green (3,3,1),
    max_3x3_blue (3,3,2),
    max_5x5_red (5,5,0),
    max_5x5_green (5,5,1),
    max_5x5_blue (5,5,2),
    max_7x7_red (7,7,0),
    max_7x7_green (7,7,1),
    max_7x7_blue (7,7,2),
    max_9x9_red (9,9,0),
    max_9x9_green (9,9,1),
    max_9x9_blue (9,9,2)
}}
