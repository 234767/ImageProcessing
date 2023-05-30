use num::complex::Complex;
use std::f64::consts::PI;
use std::ops::Mul;
use FTDirection::*;

type TData = f64;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FTDirection {
    Forward,
    Inverse,
}

#[allow(non_snake_case)]
#[inline(always)] // inline to allow for loop hoisting
fn twiddle_factor(n: usize, k: usize, N: usize, direction: FTDirection) -> Complex<TData> {
    let angle = match direction {
        Forward => -1.0,
        Inverse => 1.0,
    } * 2.0
        * PI
        * n as TData
        * (k as TData / N as TData);
    Complex::new(angle.cos(), angle.sin())
}

pub fn dft<T>(samples: &[T], direction: FTDirection) -> Vec<Complex<TData>>
where
    T: Mul<Complex<TData>, Output = Complex<TData>> + Copy,
{
    let n = samples.len();

    let result: Vec<_> = (0..n)
        .map(|k| {
            samples
                .iter()
                .enumerate()
                .map(|(j, &sample)| sample * twiddle_factor(j, k, n, direction))
                .sum()
        })
        .map(|x| match direction {
            Forward => x,
            Inverse => x / n as f64,
        })
        .collect();

    debug_assert_eq!(n, result.len());
    return result;
}

type Vec2D<T> = Vec<Vec<T>>;

pub fn dft_2d<T>(samples: &Vec<Vec<T>>, direction: FTDirection) -> Vec2D<Complex<TData>>
where
    T: Mul<Complex<TData>, Output = Complex<TData>> + Copy,
{
    let size_y = samples.len();
    let size_x = samples[0].len();
    assert!(samples.iter().all(|x| x.len() == size_x));

    let horizontal_pass: Vec2D<_> = samples.iter().map(|x| dft(x, direction)).collect();
    let mut result = vec![vec![Complex::default(); size_x]; size_y];

    for column in 0..size_x {
        let data: Vec<_> = horizontal_pass.iter().map(|row| row[column]).collect();
        for (row, value) in dft(&data, direction).into_iter().enumerate() {
            result[row][column] = value;
        }
    }

    debug_assert_eq!(result.len(), size_y);
    debug_assert!(result.iter().all(|x| Vec::len(x) == size_x));
    return result;
}

mod helpers {
    fn ilog2(value: u32) -> u32 {
        if value == 0 {
            panic!("Cannot compute logarithm of 0");
        }
        let mut value = value;
        let mut result = 0u32;
        while value > 1 {
            value >>= 1;
            result += 1;
        }
        result
    }

    fn reverse_bits(number: u32, number_of_bits: u32) -> u32 {
        let mut reversed = 0u32;
        for i in 0..number_of_bits {
            if (number & (1 << i)) != 0 {
                reversed |= 1 << (number_of_bits - i - 1);
            }
        }
        return reversed;
    }

    pub fn create_indices_rearranging_function(data_length: u32) -> impl Fn(usize) -> usize {
        let num_bits = ilog2(data_length);
        return move |x: usize| reverse_bits(x as u32, num_bits) as usize;
    }

    #[cfg(test)]
    mod unit_tests {
        use super::reverse_bits;

        #[test]
        fn test_reverse_bits() {
            let value: u32 = 0b0101;
            let num_bits: u32 = 4;
            let expected: u32 = 0b1010;

            assert_eq!(expected, reverse_bits(value, num_bits))
        }
    }
}

fn rearrange_data_for_fft<T>(data: &[T]) -> Vec<T>
where
    T: Copy,
{
    let get_index = helpers::create_indices_rearranging_function(data.len() as u32);

    let mut result = vec![];
    for i in 0..data.len() {
        result.push(data[get_index(i)]);
    }

    result
}

fn butterfly_operation(
    a: &Complex<TData>,
    b: &Complex<TData>,
    twiddle_factor: Complex<TData>,
) -> (Complex<TData>, Complex<TData>) {
    let wb = b * twiddle_factor;
    (a + wb, a - wb)
}

fn fft_in_place(data: &mut [Complex<TData>], direction: FTDirection) {
    if data.len() == 1 {
        return;
    }

    let (half_1, half_2) = data.split_at_mut(data.len() / 2);
    fft_in_place(half_1, direction);
    fft_in_place(half_2, direction);

    for i in 0..(data.len() / 2) {
        let angle = match direction {
            Forward => -2.0,
            Inverse => 2.0
        } * PI * i as f64 / data.len() as f64;
        let twiddle_factor = Complex::from_polar(1.0, angle);
        let (a, b) = butterfly_operation(&data[i], &data[i + data.len() / 2], twiddle_factor);
        data[i] = a;
        data[i + data.len() / 2] = b;
    }
}

pub fn fft<T>(data: &[T], direction: FTDirection) -> Vec<Complex<TData>>
where
    T: Mul<Complex<TData>, Output = Complex<TData>> + Copy,
{
    let mut data: Vec<_> = rearrange_data_for_fft(data)
        .into_iter()
        .map(|x| x * Complex::new(1.0, 0.0))
        .collect();

    fft_in_place(data.as_mut_slice(), direction);

    if direction == Inverse {
        let n = data.len() as f64;
        for d in &mut data {
            *d /= n
        }
    }

    data
}

pub fn fft_2d<T>(samples: &Vec2D<T>, direction: FTDirection) -> Vec2D<Complex<TData>>
    where
        T: Mul<Complex<TData>, Output = Complex<TData>> + Copy,
{
    let size_y = samples.len();
    let size_x = samples[0].len();
    assert!(samples.iter().all(|x| x.len() == size_x));

    let horizontal_pass: Vec2D<_> = samples.iter().map(|x| fft(x, direction)).collect();
    let mut result = vec![vec![Complex::default(); size_x]; size_y];

    for column in 0..size_x {
        let data: Vec<_> = horizontal_pass.iter().map(|row| row[column]).collect();
        for (row, value) in fft(&data, direction).into_iter().enumerate() {
            result[row][column] = value;
        }
    }

    debug_assert_eq!(result.len(), size_y);
    debug_assert!(result.iter().all(|x| Vec::len(x) == size_x));
    return result;
}
