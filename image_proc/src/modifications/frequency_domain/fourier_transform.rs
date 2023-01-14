use num::complex::{Complex, Complex32};
use std::f32::consts::PI;

pub fn dft_1d(samples: &[f32]) -> Vec<Complex32> {
    let n = samples.len();
    let mut result = Vec::new();

    for k in 0..n {
        let mut sum = Complex::new(0.0, 0.0);
        for j in 0..n {
            let frequency_factor = k as f32 / n as f32;
            let exponent = -2.0 * PI * j as f32 * frequency_factor;
            let direction_vector = Complex::new(0.0, exponent).exp();
            sum += samples[j]
                * direction_vector;
        }
        result.push(sum);
    }

    debug_assert_eq!(samples.len(), result.len());
    return result;
}
