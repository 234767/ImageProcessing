use num::complex::Complex;
use std::f64::consts::PI;

type TData = f64;

#[allow(non_snake_case)]
#[inline(always)] // inline to allow for loop hoisting
fn twiddle_factor(n: usize, k: usize, N: usize) -> Complex<TData> {
    let angle = -2.0 * PI * n as TData * (k as TData / N as TData);
    Complex::new(angle.cos(), angle.sin())
}

#[allow(non_snake_case)]
#[inline(always)] // inline to allow for loop hoisting
fn inverse_twiddle_factor(n: usize, k: usize, N: usize) -> Complex<TData> {
    let angle = 2.0 * PI * n as TData * (k as TData / N as TData);
    Complex::new(angle.cos(), angle.sin())
}

pub fn dft(samples: &[TData]) -> Vec<Complex<TData>> {
    let n = samples.len();

    let result: Vec<_> = (0..n)
        .map(|k| {
            samples
                .iter()
                .enumerate()
                .map(|(j, sample)| sample * twiddle_factor(j, k, n))
                .sum()
        })
        .collect();

    debug_assert_eq!(samples.len(), result.len());
    return result;
}

pub fn inverse_dft(samples: &[Complex<TData>]) -> Vec<TData> {
    let n = samples.len();

    let result: Vec<_> = (0..n)
        .map(|k| {
            samples
                .iter()
                .enumerate()
                .map(|(j, sample)| sample * inverse_twiddle_factor(j, k, n))
                .map(|x| x.re)
                .sum::<TData>()
                / n as TData
        })
        .collect();

    debug_assert_eq!(samples.len(), result.len());
    return result;
}
