use image_proc::modifications::frequency_domain::fourier_transform::{dft, FFTDirection};
use num::Complex;
use super::ACCURACY;

fn test_dft_1d(input: &[f64], expected: &[(f64, f64)]) {
    let expected: Vec<_> = expected
        .iter()
        .map(|(re, im)| Complex::new(*re, *im))
        .collect();

    let result = dft(input, FFTDirection::Forward);

    for (expected, result) in expected.iter().zip(result) {
        assert_delta!(expected.re, result.re, ACCURACY);
        assert_delta!(expected.im, result.im, ACCURACY);
    }
}

fn test_inverse_dft_1d(input: &[(f64, f64)], expected: &[f64]) {
    let input: Vec<_> = input
        .iter()
        .map(|(re, im)| Complex::new(*re, *im))
        .collect();

    let result = dft(&input, FFTDirection::Inverse).into_iter().map(|x| x.re);

    for (expected, result) in expected.iter().zip(result) {
        assert_delta!(expected, result, ACCURACY);
    }
}

invoke_test! { test_dft_1d {
    dft_forward_1 (&[10.0,0.0,-10.0,0.0], &[(0.0,0.0),(20.0,0.0),(0.0,0.0),(20.0,0.0)]),
    dft_forward_2 (&[2.0,-2.0,1.0,-1.0], &[(0.0,0.0),(1.0,1.0),(6.0,0.0),(1.0,-1.0)])
}}

invoke_test! { test_inverse_dft_1d {
    dft_inverse_1 (&[(0.0,0.0),(20.0,0.0),(0.0,0.0),(20.0,0.0)], &[10.0,0.0,-10.0,0.0]),
    dft_inverse_2 ( &[(0.0,0.0),(1.0,1.0),(6.0,0.0),(1.0,-1.0)], &[2.0,-2.0,1.0,-1.0]),
}}
