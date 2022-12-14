use super::{is_foreground, BACKGROUND_PIXEL, FOREGROUND_PIXEL};
use image::{GrayImage, ImageBuffer, Luma, Pixel};
use std::ops::Deref;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mask {
    data: u16, // 9 bits needed, so 2 bytes
}

fn is_unwritable<P, Container>(
    image: &ImageBuffer<P, Container>,
    x: u32,
    y: u32,
    i: u32,
    j: u32,
) -> bool
where
    P: Pixel,
    Container: Deref<Target = [P::Subpixel]>,
{
    x + i == 0 || y + i == 0 || x + i > image.width() || y + j > image.height()
}

impl Mask {
    pub fn new() -> Self {
        Self { data: 0u16 }
    }

    pub fn from_image(image: &GrayImage, x: u32, y: u32) -> Self {
        let mut mask = Self::new();
        for i in 0..3 {
            for j in 0..3 {
                if is_unwritable(image, x, y, i, j) {
                    continue;
                }
                mask.set_pixel(i, j, image.get_pixel(x + i - 1, y + j - 1));
            }
        }
        mask
    }

    pub fn write_to_image(&self, image: &mut GrayImage, x: u32, y: u32) {
        for i in 0..3 {
            for j in 0..3 {
                if is_unwritable(image, x, y, i, j) {
                    continue;
                }
                image.put_pixel(x + i - 1, y + j - 1, self.get_pixel(i, j))
            }
        }
    }

    pub fn set_bit(&mut self, x: u32, y: u32) {
        debug_assert!(x < 3 && y < 3);
        let bit_mask = 1 << (y * 3 + x);
        self.data |= bit_mask;
    }

    pub fn clear_bit(&mut self, x: u32, y: u32) {
        debug_assert!(x < 3 && y < 3);
        let bit_mask = 1 << (y * 3 + x);
        self.data &= !bit_mask;
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: &Luma<u8>) {
        if is_foreground(pixel) {
            self.set_bit(x, y);
        } else {
            self.clear_bit(x, y);
        }
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Luma<u8> {
        debug_assert!(x < 3 && y < 3);
        let bit_mask = 1 << (y * 3 + x);
        let foreground = &self.data & bit_mask != 0;
        match foreground {
            true => FOREGROUND_PIXEL,
            false => BACKGROUND_PIXEL,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.data == 0
    }
}

impl std::ops::BitAnd<Mask> for Mask {
    type Output = Self;

    fn bitand(self, rhs: Mask) -> Self::Output {
        Self::Output {
            data: self.data & rhs.data,
        }
    }
}

impl std::ops::BitOr<Mask> for Mask {
    type Output = Self;

    fn bitor(self, rhs: Mask) -> Self::Output {
        Self::Output {
            data: self.data | rhs.data,
        }
    }
}
