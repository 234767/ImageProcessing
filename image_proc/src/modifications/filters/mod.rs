macro_rules! impl_new {
    () => {
        pub fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    };
}

pub mod basic;
pub mod linear;
pub mod nonlinear;

pub use basic::*;
pub use linear::*;
pub use nonlinear::*;

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

            let result =
                Neighbourhood::new(&image, x, x_offset, y, y_offset).non_enumerated_count();

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
}
