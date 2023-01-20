use num::complex::Complex;
use std::f64::consts::PI;
use std::ops::Mul;

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

pub fn dft<T>(samples: &[T]) -> Vec<Complex<TData>>
where
    T: Mul<Complex<TData>, Output = Complex<TData>> + Copy,
{
    let n = samples.len();

    let result: Vec<_> = (0..n)
        .map(|k| {
            samples
                .iter()
                .enumerate()
                .map(|(j, &sample)| sample * twiddle_factor(j, k, n))
                .sum()
        })
        .collect();

    debug_assert_eq!(n, result.len());
    return result;
}

pub fn inverse_dft(samples: &[Complex<TData>]) -> Vec<Complex<TData>> {
    let n = samples.len();

    let result: Vec<_> = (0..n)
        .map(|k| {
            samples
                .iter()
                .enumerate()
                .map(|(j, sample)| sample * inverse_twiddle_factor(j, k, n))
                .sum::<Complex<TData>>()
                / n as TData
        })
        .collect();

    debug_assert_eq!(n, result.len());
    return result;
}

type Vec2D<T> = Vec<Vec<T>>;

pub fn dft_2d<T>(samples: &[&[T]]) -> Vec2D<Complex<TData>>
where
    T: Mul<Complex<TData>, Output = Complex<TData>> + Copy,
{
    let size_y = samples.len();
    let size_x = samples[0].len();
    assert!(samples.iter().all(|x| x.len() == size_x));

    let horizontal_pass: Vec2D<_> = samples.iter().map(|x| dft(x)).collect();
    let mut result = vec![vec![Complex::default(); size_x]; size_y];

    for column in 0..size_x {
        let data: Vec<_> = horizontal_pass.iter().map(|row| row[column]).collect();
        for (row, value) in dft(&data).into_iter().enumerate() {
            result[row][column] = value;
        }
    }

    debug_assert_eq!(result.len(), size_y);
    debug_assert!(result.iter().all(|x| Vec::len(x) == size_x));
    return result;
}

pub fn inverse_dft_2d(samples: &[&[Complex<TData>]]) -> Vec2D<Complex<TData>> {
    let size_y = samples.len();
    let size_x = samples[0].len();
    assert!(samples.iter().all(|x| x.len() == size_x));

    let horizontal_pass: Vec2D<_> = samples.iter().map(|x| inverse_dft(x)).collect();
    let mut result = vec![vec![Complex::default(); size_x]; size_y];

    for column in 0..size_x {
        let data: Vec<_> = horizontal_pass.iter().map(|row| row[column]).collect();
        for (row, value) in inverse_dft(&data).into_iter().enumerate() {
            result[row][column] = value;
        }
    }

    debug_assert_eq!(result.len(), size_y);
    debug_assert!(result.iter().all(|x| Vec::len(x) == size_x));
    return result;
}