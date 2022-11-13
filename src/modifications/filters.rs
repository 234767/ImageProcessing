//Methods of image noise removal
use crate::modifications::Transformation;
use crate::parsing::Args;
use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;

use num::pow::Pow;
use num::Integer;

pub mod gpu_optimized;

fn is_in_range(x: u32, y: u32, image: &RgbImage) -> bool {
    x < image.width() && y < image.height()
}

fn collect_pixels(image: &RgbImage, x: u32, x_offset: u32, y: u32, y_offset: u32) -> Vec<&Rgb<u8>> {
    let mut old_pixels: Vec<&Rgb<u8>> = Vec::new();
    for x in u32::saturating_sub(x, x_offset)..=(x + x_offset) {
        for y in u32::saturating_sub(y, y_offset)..=(y + y_offset) {
            if is_in_range(x, y, image) {
                old_pixels.push(image.get_pixel(x, y));
            }
        }
    }
    old_pixels
}

fn get_width_and_height(args: &Args) -> Result<(u32, u32), String> {
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

//(N1) Median filter (--median)
pub struct MedianFilter {
    width: u32,
    height: u32,
}

impl MedianFilter {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let (width, height) = get_width_and_height(args)?;
        Ok(Self { width, height })
    }
}

impl Transformation for MedianFilter {
    fn apply(&self, image: &mut RgbImage) {
        let width_offset = self.width / 2;
        let height_offset = self.height / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let old_pixels: Vec<&Rgb<u8>> =
                collect_pixels(image, target_x, width_offset, target_y, height_offset);
            for channel in 0..3 {
                let mut luminosities: Vec<u8> =
                    old_pixels.iter().map(|pixel| pixel[channel]).collect();
                luminosities.sort();
                new_pixel[channel] = luminosities[luminosities.len() / 2];
            }
        }
        *image = new_image;
    }
}

//(N1) geometric mean filter (--gmean)
pub struct GeometricMeanFilter {
    width: u32,
    height: u32,
}

impl GeometricMeanFilter {
    pub fn try_new(args: &Args) -> Result<Self, String> {
        let (width, height) = get_width_and_height(args)?;
        Ok(Self { width, height })
    }
}

impl Transformation for GeometricMeanFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let old_pixels: Vec<&Rgb<u8>> =
                collect_pixels(image, target_x, w_offset, target_y, h_offset);
            for channel in 0..3 {
                let luminosities: Vec<u8> = old_pixels.iter().map(|pixel| pixel[channel]).collect();
                new_pixel[channel] = f64::pow(
                    luminosities.iter().map(|l| *l as f64).product::<f64>(),
                    1f64 / luminosities.iter().count() as f64,
                ) as u8;
            }
        }
        *image = new_image;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    pub struct FilterTestFixture {
        pub image: RgbImage,
    }

    impl FilterTestFixture {
        pub fn new() -> Self {
            // image size 10x10
            let width = 10;
            let height = 10;
            let buffer: Vec<u8> = (0..(width * height * 3)).map(|c| c as u8).collect();
            let image: RgbImage = RgbImage::from_vec(width, height, buffer).unwrap();
            Self { image }
        }
    }

    #[test]
    fn fixture_setup_correctly() {
        let _ = FilterTestFixture::new();
    }

    #[test]
    fn collect_pixels_works_in_the_middle() {
        let fixture = FilterTestFixture::new();
        let x = 3;
        let y = 4;
        let x_offset = 1;
        let y_offset = 2;

        let result = collect_pixels(&fixture.image, x, x_offset, y, y_offset);

        assert_eq!(15, result.len());

        for xi in x - x_offset..=x + x_offset {
            for yi in y - y_offset..=y + y_offset {
                let pixel = fixture.image.get_pixel(xi, yi);
                assert!(result.contains(&pixel));
            }
        }
    }

    #[test]
    fn collect_pixels_works_on_edges() {
        let image = FilterTestFixture::new().image;
        let x = 0;
        let y = 1;
        let x_offset = 1;
        let y_offset = 2;

        let result = collect_pixels(&image, x, x_offset, y, y_offset);

        assert_eq!(8, result.len());

        for xi in 0..=x + x_offset {
            for yi in 0..y + y_offset {
                let pixel = image.get_pixel(xi, yi);
                assert!(result.contains(&pixel));
            }
        }
    }

    mod median_filter_tests {
        use super::*;

        macro_rules! cpu_tests {
            ($($name:ident $values:expr,)*) => {
                $(
                #[test]
                fn $name () {
                    let (width, height, channel) = $values;
                    let mut image = FilterTestFixture::new().image;
                    let filter = MedianFilter { width, height };

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
                    let (target_x, target_y) = ((1+width) / 2, (1+height)/2);
                    let Rgb(target_pixel) = image.get_pixel(target_x, target_y);
                    assert_eq!(median, target_pixel[channel]);
                }
                )*
            }
        }

        cpu_tests! {
            median_3x3_cpu_red (3,3,0),
            median_3x3_cpu_green (3,3,1),
            median_3x3_cpu_blue (3,3,2),
            median_5x5_cpu_red (5,5,0),
            median_5x5_cpu_green (5,5,1),
            median_5x5_cpu_blue (5,5,2),
            median_7x7_cpu_red (7,7,0),
            median_7x7_cpu_green (7,7,1),
            median_7x7_cpu_blue (7,7,2),
            median_9x9_cpu_red (9,9,0),
            median_9x9_cpu_green (9,9,1),
            median_9x9_cpu_blue (9,9,2),
        }
    }
}
