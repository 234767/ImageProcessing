use image_proc::modifications::frequency_domain::fourier_transform::{fft, FTDirection};
use num::Complex;
use super::ACCURACY;

fn test_fft_1d(input: &[f64], expected: &[(f64, f64)]) {
    let expected: Vec<_> = expected
        .iter()
        .map(|(re, im)| Complex::new(*re, *im))
        .collect();

    let result = fft(&input, FTDirection::Forward);

    for (expected, result) in expected.iter().zip(result) {
        assert_delta!(expected.re, result.re, ACCURACY);
        assert_delta!(expected.im, result.im, ACCURACY);
    }
}

fn test_inverse_fft_1d(input: &[(f64, f64)], expected: &[f64]) {
    let input: Vec<_> = input
        .iter()
        .map(|(re, im)| Complex::new(*re, *im))
        .collect();

    let result = fft(&input, FTDirection::Inverse).into_iter().map(|x| x.re);

    for (expected, result) in expected.iter().zip(result) {
        assert_delta!(expected, result, ACCURACY);
    }
}

invoke_test! { test_fft_1d {
    fft_forward_1 (&[10.0,0.0,-10.0,0.0], &[(0.0,0.0),(20.0,0.0),(0.0,0.0),(20.0,0.0)]),
    fft_forward_2 (&[2.0,-2.0,1.0,-1.0], &[(0.0,0.0),(1.0,1.0),(6.0,0.0),(1.0,-1.0)]),
    fft_forward_3 (&[2.0,1.0,-1.0,5.0,0.0,3.0,0.0,-4.0], &[(6.0,0.0),(-5.778,-3.95),(3.0,-3.0),(9.778,-5.95),(-4.0,0.0),(9.778,5.95),(3.0,3.0),(-5.778,3.95)])
}}

invoke_test! { test_inverse_fft_1d {
    fft_inverse_1 (&[(0.0,0.0),(20.0,0.0),(0.0,0.0),(20.0,0.0)], &[10.0,0.0,-10.0,0.0]),
    fft_inverse_2 ( &[(0.0,0.0),(1.0,1.0),(6.0,0.0),(1.0,-1.0)], &[2.0,-2.0,1.0,-1.0]),
    fft_inverse_3 (&[(6.0,0.0),(-5.778,-3.95),(3.0,-3.0),(9.778,-5.95),(-4.0,0.0),(9.778,5.95),(3.0,3.0),(-5.778,3.95)], &[2.0,1.0,-1.0,5.0,0.0,3.0,0.0,-4.0])
}}
