use super::super::Transformation;
use super::iterating::Neighbourhood;
use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;
use num::pow::Pow;

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

pub struct MinimumFilter {
    width: u32,
    height: u32,
}

impl MinimumFilter {
    impl_new!();
}

impl Transformation for MinimumFilter {
    fn apply(&self, image: &mut RgbImage) {
        let h_offset = self.height / 2;
        let w_offset = self.width / 2;
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (target_x, target_y, new_pixel) in new_image.enumerate_pixels_mut() {
            let neighbourhood = Neighbourhood::new(image, target_x, w_offset, target_y, h_offset);
            let min_values =
                neighbourhood
                    .iter()
                    .fold([255u8, 255u8, 255u8], |prod, Rgb(pixel)| {
                        [
                            u8::min(prod[0], pixel[0]),
                            u8::min(prod[1], pixel[1]),
                            u8::min(prod[2], pixel[2]),
                        ]
                    });
            *new_pixel = Rgb::from(min_values)
        }
        *image = new_image;
    }
}

fn is_edge(image: &RgbImage, x: u32, y: u32) -> bool {
    0 == x || x == image.width() - 1 || 0 == y || y == image.height() - 1
}

pub struct Uolis;

impl Transformation for Uolis {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = ImageBuffer::new(image.width(), image.height());
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            if is_edge(image, x, y) {
                continue;
            }
            let neighbors = {
                let mut neighbors: Vec<&Rgb<u8>> = vec![];
                for (i, j) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let xi = (x as i32 + i) as u32;
                    let yi = (y as i32 + j) as u32;
                    if xi < image.width() && yi < image.height() {
                        neighbors.push(image.get_pixel(xi, yi));
                    }
                }
                neighbors
            };
            for channel in 0..3 {
                let product = neighbors.iter().map(|x| x[channel] as f64).product::<f64>();
                let power = (image.get_pixel(x, y)[channel] as f64).pow(4.0);
                let log_base = power / product;
                let log = f64::log10(log_base);
                pixel[channel] = (log / 4.0) as u8
            }
        }
        *image = new_image;
    }
}

pub struct LowPassFilter {
    mask: [[f64; 3]; 3],
    mask_scale: f64,
}

impl LowPassFilter {
    pub fn new(mask: [[f64; 3]; 3], mask_scale: Option<f64>) -> Self {
        Self {
            mask,
            mask_scale: mask_scale.unwrap_or(1.0),
        }
    }
    pub fn from_flat_mask(flat_mask: [f64; 9], mask_scale: Option<f64>) -> Self {
        let mut mask = [[0f64; 3]; 3];
        for y in 0..3 {
            for x in 0..3 {
                mask[y][x] = flat_mask[3 * y + x]
            }
        }
        Self {
            mask,
            mask_scale: mask_scale.unwrap_or(1.0),
        }
    }
}

impl Transformation for LowPassFilter {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = RgbImage::new(image.width(), image.height());
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            if is_edge(image, x, y) {
                *pixel = *image.get_pixel(x,y);
                continue;
            }

            for channel in 0..3 {
                let mut sum = 0f64;
                for i in 0..3 {
                    for j in 0..3 {
                        sum += (self.mask[j as usize][i as usize])
                            * image.get_pixel(x + i - 1, y + j - 1)[channel] as f64;
                    }
                }
                pixel[channel] = (sum * self.mask_scale) as u8;
            }
        }
        *image = new_image;
    }
}
