use super::ACCURACY;
use image_proc::modifications::frequency_domain::fourier_transform::{fft_2d, FTDirection};
use num::Complex;
use std::convert::identity;

fn slice_to_complex_vec_2d(input: &[&[(f64,f64)]]) -> Vec<Vec<Complex<f64>>> {
    input
        .iter()
        .map(|&x| x.iter().map(|(x, y)| Complex::new(*x, *y)).collect())
        .collect()
}

fn test_fft_2d(input: &[&[f64]], expected: &[&[(f64, f64)]]) {
    let expected = slice_to_complex_vec_2d(expected);

    let input: Vec<Vec<_>> = input.iter().map(|row| row.to_vec()).collect();

    let result = fft_2d(&input, FTDirection::Forward);

    for (expected, result) in expected
        .iter()
        .flat_map(identity)
        .zip(result.iter().flat_map(identity))
    {
        assert_delta!(expected.re, result.re, ACCURACY);
        assert_delta!(expected.im, result.im, ACCURACY);
    }
}

fn test_inverse_fft_2d(input: &[&[(f64, f64)]], expected: &[&[f64]]) {
    let input = slice_to_complex_vec_2d(input);

    let result = fft_2d(&input, FTDirection::Inverse);

    for (expected, result) in expected
        .iter()
        .flat_map(|&x| x)
        .zip(result.iter().flat_map(|x| x.into_iter().map(|x|x.re)))
    {
        assert_delta!(expected, result, ACCURACY);
        assert_delta!(expected, result, ACCURACY);
    }
}

invoke_test! { test_fft_2d {
    fft_forward_1 (&[&[1.0,2.0],&[3.0,4.0]], &[&[(10.0,0.0),(-2.0,0.0)],&[(-4.0,0.0),(0.0,0.0)]]),
}}

invoke_test!{ test_inverse_fft_2d {
    fft_inverse_1 (&[&[(10.0,0.0),(-2.0,0.0)],&[(-4.0,0.0),(0.0,0.0)]], &[&[1.0,2.0],&[3.0,4.0]]),
}}