use super::ACCURACY;
use image_proc::modifications::frequency_domain::fourier_transform::{dft_2d, FTDirection};
use num::Complex;
use std::convert::identity;

fn slice_to_complex_vec_2d(input: &[&[(f64,f64)]]) -> Vec<Vec<Complex<f64>>> {
    input
        .iter()
        .map(|&x| x.iter().map(|(x, y)| Complex::new(*x, *y)).collect())
        .collect()
}

fn test_dft_2d(input: &[&[f64]], expected: &[&[(f64, f64)]]) {
    let expected = slice_to_complex_vec_2d(expected);

    let result = dft_2d(input, FTDirection::Forward);

    for (expected, result) in expected
        .iter()
        .flat_map(identity)
        .zip(result.iter().flat_map(identity))
    {
        assert_delta!(expected.re, result.re, ACCURACY);
        assert_delta!(expected.im, result.im, ACCURACY);
    }
}

fn test_inverse_dft_2d(input: &[&[(f64, f64)]], expected: &[&[f64]]) {
    let input = slice_to_complex_vec_2d(input);

    let input_as_slice: Vec<&[_]> = input.iter().map(|x|x.as_slice()).collect();

    let result = dft_2d(&input_as_slice, FTDirection::Inverse);

    for (expected, result) in expected
        .iter()
        .flat_map(|&x| x)
        .zip(result.iter().flat_map(|x| x.into_iter().map(|x|x.re)))
    {
        assert_delta!(expected, result, ACCURACY);
        assert_delta!(expected, result, ACCURACY);
    }
}

invoke_test! { test_dft_2d {
    dft_forward_1 (&[&[1.0,2.0],&[3.0,4.0]], &[&[(10.0,0.0),(-2.0,0.0)],&[(-4.0,0.0),(0.0,0.0)]]),
}}

invoke_test!{ test_inverse_dft_2d {
    dft_inverse_1 (&[&[(10.0,0.0),(-2.0,0.0)],&[(-4.0,0.0),(0.0,0.0)]], &[&[1.0,2.0],&[3.0,4.0]]),
}}