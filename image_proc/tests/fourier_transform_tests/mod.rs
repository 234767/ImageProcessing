const ACCURACY: f64 = 1e-6;

macro_rules! assert_delta {
    ($x:expr, $y:expr, $d:expr) => {
        if !($x - $y < $d || $y - $x < $d) {
            panic!();
        }
    };
}

mod dft_1d_tests;
mod dtt_2d_tests;
