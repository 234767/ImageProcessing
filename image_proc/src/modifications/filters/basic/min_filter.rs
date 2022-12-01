use crate::modifications::filters::iterating::Neighbourhood;
use crate::modifications::Transformation;
use image::{ImageBuffer, Rgb, RgbImage};

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
