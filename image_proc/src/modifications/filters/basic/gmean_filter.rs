use image::{ImageBuffer, Rgb, RgbImage};
use num::pow::Pow;
use crate::modifications::filters::iterating::Neighbourhood;
use crate::modifications::Transformation;

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
