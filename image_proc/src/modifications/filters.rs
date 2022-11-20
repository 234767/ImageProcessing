//Methods of image noise removal
use super::Transformation;
use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;
use num::pow::Pow;

pub mod gpu_optimized;

macro_rules! impl_new {
    () => {
        pub fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    };
}

mod iterating {
    use image::{Rgb, RgbImage};

    pub struct Neighbourhood<'a> {
        image: &'a RgbImage,
        min_x: u32,
        max_x: u32,
        min_y: u32,
        max_y: u32,
    }

    impl<'a> Neighbourhood<'a> {
        pub fn new(image: &'a RgbImage, x: u32, x_offset: u32, y: u32, y_offset: u32) -> Self {
            let min_x = u32::min(u32::saturating_sub(x, x_offset), image.width() - 1);
            let min_y = u32::min(u32::saturating_sub(y, y_offset), image.height() - 1);
            let max_x = u32::min(x + x_offset, image.width() - 1);
            let max_y = u32::min(y + y_offset, image.height() - 1);
            Self {
                image,
                min_x,
                max_x,
                min_y,
                max_y,
            }
        }

        pub fn iter(&self) -> NeighbourhoodIterator<'_, 'a> {
            NeighbourhoodIterator {
                neighbourhood: self,
                x: self.min_x,
                y: self.min_y,
            }
        }
    }

    impl Neighbourhood<'_> {
        pub fn non_enumerated_count(&self) -> usize {
            ((self.max_x - self.min_x + 1) * (self.max_y - self.min_y + 1)) as usize
        }
    }

    pub struct NeighbourhoodIterator<'n, 'img: 'n> {
        neighbourhood: &'n Neighbourhood<'img>,
        x: u32,
        y: u32,
    }

    impl NeighbourhoodIterator<'_, '_> {
        fn advance(&mut self) {
            self.x += 1;
            if self.x > self.neighbourhood.max_x {
                // go to next row
                self.y += 1;
                self.x = self.neighbourhood.min_x;
            }
        }

        fn is_finished(&self) -> bool {
            self.y > self.neighbourhood.max_y
        }
    }

    impl<'a> Iterator for NeighbourhoodIterator<'_, 'a> {
        type Item = &'a Rgb<u8>;

        fn next(&mut self) -> Option<Self::Item> {
            if self.is_finished() {
                return None;
            }
            let pixel = self.neighbourhood.image.get_pixel(self.x, self.y);
            self.advance();
            Some(pixel)
        }
    }
}
use iterating::Neighbourhood;

//(N1) Median filter (--median)
pub struct MedianFilter {
    width: u32,
    height: u32,
}

impl MedianFilter {
    impl_new!();
}

impl Transformation for MedianFilter {
    fn apply(&self, image: &mut RgbImage) {
        let width_offset = self.width / 2;
        let height_offset = self.height / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let mut luminosity_buckets = [[0u32; 256]; 3];
            let neighbourhood =
                Neighbourhood::new(image, target_x, width_offset, target_y, height_offset);
            for Rgb(pixel) in neighbourhood.iter() {
                for channel in 0..3 {
                    let luminosity = pixel[channel];
                    luminosity_buckets[channel][luminosity as usize] += 1;
                }
            }
            for channel in 0..3 {
                let luminosity_buckets = luminosity_buckets[channel];
                let median_index = neighbourhood.non_enumerated_count() as u32 / 2;
                let median: u8 = {
                    let mut median: u8 = 255;
                    let mut partial_sum: u32 = 0;
                    for l in 0..=255 {
                        partial_sum += luminosity_buckets[l];
                        if partial_sum > median_index {
                            median = l as u8;
                            break;
                        }
                    }
                    median
                };
                new_pixel[channel] = median;
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
    impl_new!();
}

impl Transformation for GeometricMeanFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let neighbourhood = Neighbourhood::new(image, target_x, w_offset, target_y, h_offset);
            let products = neighbourhood
                .iter()
                .fold([1.0, 1.0, 1.0], |prod, Rgb(pixel)| {
                    [
                        prod[0] * pixel[0] as f64,
                        prod[1] * pixel[1] as f64,
                        prod[2] * pixel[2] as f64,
                    ]
                });
            for channel in 0..3 {
                new_pixel[channel] = f64::pow(
                    products[channel],
                    1f64 / neighbourhood.non_enumerated_count() as f64,
                ) as u8;
            }
        }
        *image = new_image;
    }
}

pub struct MaxFilter {
    width: u32,
    height: u32,
}

impl MaxFilter {
    impl_new!();
}

impl Transformation for MaxFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, Rgb(new_pixel)) in new_image.enumerate_pixels_mut() {
            let neighbourhood = Neighbourhood::new(image, target_x, w_offset, target_y, h_offset);
            let max_values = neighbourhood.iter().fold([0u8, 0, 0], |prod, Rgb(pixel)| {
                [
                    u8::max(prod[0], pixel[0]),
                    u8::max(prod[1], pixel[1]),
                    u8::max(prod[2], pixel[2]),
                ]
            });
            *new_pixel = max_values
        }
        *image = new_image;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct FilterTestFixture {
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
    fn neighborhood_iter_works_in_the_middle() {
        let image = FilterTestFixture::new().image;
        let x = 3;
        let y = 4;
        let x_offset = 1;
        let y_offset = 2;

        let result: Vec<_> = Neighbourhood::new(&image, x, x_offset, y, y_offset)
            .iter()
            .collect();

        assert_eq!(15, result.len());

        for xi in x - x_offset..=x + x_offset {
            for yi in y - y_offset..=y + y_offset {
                let pixel = image.get_pixel(xi, yi);
                assert!(result.contains(&pixel));
            }
        }
    }

    #[test]
    fn neighborhood_iter_works_on_edges() {
        let image = FilterTestFixture::new().image;
        let x = 0;
        let y = 1;
        let x_offset = 1;
        let y_offset = 2;

        let result: Vec<_> = Neighbourhood::new(&image, x, x_offset, y, y_offset)
            .iter()
            .collect();

        assert_eq!(8, result.len());

        for xi in 0..=x + x_offset {
            for yi in 0..y + y_offset {
                let pixel = image.get_pixel(xi, yi);
                assert!(result.contains(&pixel));
            }
        }
    }

    #[test]
    fn neighbourhood_non_enumerated_count_works_in_center() {
        let image = FilterTestFixture::new().image;
        let x = 3;
        let y = 4;
        let x_offset = 1;
        let y_offset = 2;

        let result = Neighbourhood::new(&image, x, x_offset, y, y_offset).non_enumerated_count();

        assert_eq!(15, result);
    }

    #[test]
    fn neighbourhood_non_enumerated_count_works_on_edges() {
        let image = FilterTestFixture::new().image;
        let x = 9;
        let y = 9;
        let x_offset = 1;
        let y_offset = 2;

        let neighbourhood = Neighbourhood::new(&image, x, x_offset, y, y_offset);
        let result = neighbourhood.non_enumerated_count();

        assert_eq!(6, result);
    }
}
