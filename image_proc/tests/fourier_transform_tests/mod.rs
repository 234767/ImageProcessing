const ACCURACY: f64 = 1e-3;

macro_rules! assert_delta {
    ($a:expr, $b:expr, $d:expr) => {
        let (a, b) = (&$a, &$b);
        let eps = $d;
        assert!(
            (*a - *b).abs() < eps,
            "assertion failed: `(left != right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            eps,
            (*a - *b).abs()
        );
    };
}

mod dft_1d_tests;
mod dtt_2d_tests;
mod fft_1d_tests;
mod fft_2d_tests;
