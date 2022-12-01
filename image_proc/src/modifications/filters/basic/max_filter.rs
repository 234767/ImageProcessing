use image::{ImageBuffer, Rgb, RgbImage};
use crate::modifications::filters::iterating::Neighbourhood;
use crate::modifications::Transformation;

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
