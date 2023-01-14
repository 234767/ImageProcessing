use num::complex::{Complex, Complex64};
use std::f64::consts::PI;

#[allow(non_snake_case)]
#[inline(always)] // inline to allow for loop hoisting
fn twiddle_factor(n: usize, k: usize, N: usize) -> Complex<f64> {
    Complex::new(0.0, -2.0 * PI * n as f64 * (k as f64 / N as f64)).exp()
}

pub fn dft_1d(samples: &[f64]) -> Vec<Complex64> {
    let n = samples.len();

    let result: Vec<_> = (0..n).map(|k|{
        samples
            .iter()
            .enumerate()
            .map(|(j, sample)| sample * twiddle_factor(j, k, n))
            .sum()
    }).collect();

    debug_assert_eq!(samples.len(), result.len());
    return result;
}