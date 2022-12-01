use crate::modifications::Transformation;
use image::{ImageBuffer, RgbImage};

pub struct Scale {
    factor_x: f64,
    factor_y: f64,
}

//(G4) Image shrinking (--shrink) & (G5) Image enlargement (--enlarge)
impl Transformation for Scale {
    fn apply(&self, image: &mut RgbImage) {
        let mut new_image: RgbImage = ImageBuffer::new(
            (image.width() as f64 * self.factor_x) as u32,
            (image.height() as f64 * self.factor_y) as u32,
        );
        for (x, y, pixel) in new_image.enumerate_pixels_mut() {
            let (src_x, src_y) = self.src_pixel_from_target(x, y);
            *pixel = *image.get_pixel(src_x, src_y);
        }
        *image = new_image;
    }
}

impl Scale {
    pub fn new(factor_x: f64, factor_y: f64) -> Self {
        Self { factor_x, factor_y }
    }

    /// Returns a pair of `x,y` coordinates in the source image,
    /// corresponding to the specified `x,y` coordinates in the target image
    fn src_pixel_from_target(&self, target_x: u32, target_y: u32) -> (u32, u32) {
        (
            (target_x as f64 / self.factor_x) as u32,
            (target_y as f64 / self.factor_y) as u32,
        )
    }
}
