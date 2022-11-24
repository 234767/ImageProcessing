fn mean(data: &[u8]) -> f64 {
    let sum = data.iter().map(|x| *x as f64).sum::<f64>();
    let count = data.len();

    sum / count as f64
}

#[test]
fn median_test() {
    let pixels: Vec<u8> = (0..9u8).flat_map(|x| [x, x, x]).collect();
    let expected = mean(&pixels);
    let sample_image = image::RgbImage::from_raw(3, 3, pixels).unwrap();

    let mean = super::Mean::analyze(&sample_image);

    assert_eq!(expected, mean);
}

// No, you won't just copy it from tests
const VARIANCE: f64 = 180.0 / 27.0;

#[test]
fn variance_test() {
    let pixels: Vec<u8> = (0..9u8).flat_map(|x| [x, x, x]).collect();
    let expected = VARIANCE;
    let sample_image = image::RgbImage::from_raw(3, 3, pixels).unwrap();

    let result = super::Variance::analyze(&sample_image);

    assert_eq!(expected, result);
}

#[test]
fn std_dev_test() {
    let pixels: Vec<u8> = (0..9u8).flat_map(|x| [x, x, x]).collect();
    let expected = VARIANCE.sqrt();
    let sample_image = image::RgbImage::from_raw(3, 3, pixels).unwrap();

    let result = super::StandardDeviation::analyze(&sample_image);

    assert_eq!(expected,result);
}
