use image::{GrayImage, ImageBuffer, Luma};
use image_proc::modifications::morphological::{mask::Mask, MorphologicalTransform, erosion::Erosion};
use crate::morphological_operations_tests::is_foreground;

#[test]
fn erosion_test() {
    let fg = Luma::from([255u8]);
    let mut image: GrayImage = ImageBuffer::new(3, 3);
    image.put_pixel(0,2,fg);
    image.put_pixel(1,2,fg);
    image.put_pixel(1,1,fg);
    image.put_pixel(2,1,fg);
    let transformation = {
        let mut mask = Mask::new();
        mask.set_bit(1, 1);
        mask.set_bit(2, 1);
        /*
        0 0 0
        0 1 1
        0 0 0
         */
        Erosion::new(mask)
    };

    transformation.apply_morph_operation(&mut image);

    assert!(is_foreground(image.get_pixel(0,2)));
    assert!(is_foreground(image.get_pixel(1,1)));

    assert!(!is_foreground(image.get_pixel(1,2)));
    assert!(!is_foreground(image.get_pixel(2,1)));
}
