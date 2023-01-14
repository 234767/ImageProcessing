use image_proc::modifications::frequency_domain::fourier_transform::dft_1d;
use num::Complex;

const ACCURACY: f64 = 1e-6;

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) {
            panic!();
        }
    };
}

fn test_dft_1d(input: &[f64], expected: &[(f64, f64)]) {
    let expected: Vec<_> = expected
        .iter()
        .map(|(re, im)| Complex::new(*re, *im))
        .collect();

    let result = dft_1d(input);

    for (expected, result) in expected.iter().zip(result) {
        assert_delta!(expected.re, result.re, ACCURACY);
        assert_delta!(expected.im, result.im, ACCURACY);
    }
}

invoke_test! { test_dft_1d {
    run_1 (&[10.0,0.0,-10.0,0.0], &[(0.0,0.0),(20.0,0.0),(0.0,0.0),(20.0,0.0)]),
    run_2 (&[2.0,-2.0,1.0,-1.0], &[(0.0,0.0),(1.0,1.0),(6.0,0.0),(1.0,-1.0)])
}}
