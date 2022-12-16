use super::is_foreground;
use image::{GrayImage, ImageBuffer, Luma};
use image_proc::modifications::morphological::{
    hmt::HitOrMissTransform, mask::Mask, MorphologicalTransform,
};

#[test]
fn hmt_test() {
    let fg = Luma::from([255u8]);
    let mut image: GrayImage = ImageBuffer::new(3, 3);
    image.put_pixel(0, 2, fg);
    image.put_pixel(1, 2, fg);
    image.put_pixel(1, 1, fg);
    image.put_pixel(2, 1, fg);
    image.put_pixel(1, 0, fg);
    let transformation = {
        let mut hits = Mask::new();
        hits.set_bit(1, 1);
        hits.set_bit(1, 0);
        /*
        0 1 0
        0 1 0
        0 0 0
         */

        let mut misses = Mask::new();
        misses.set_bit(2, 1);
        /*
        0 0 0
        0 0 1
        0 0 0
         */
        HitOrMissTransform::new(hits, misses)
    };

    transformation.apply_morph_operation(&mut image);

    // Foreground pixels
    assert!(is_foreground(image.get_pixel(1, 2)));

    //Background pixels
    assert!(!is_foreground(image.get_pixel(0, 2)));
    assert!(!is_foreground(image.get_pixel(1, 1)));
    assert!(!is_foreground(image.get_pixel(2, 1)));
    assert!(!is_foreground(image.get_pixel(1, 0)));
}
