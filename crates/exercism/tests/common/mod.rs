#![allow(dead_code)]

pub fn assert_approx_equal_numbers(expected: f64, actual: f64) {
    let diff: f64 = (expected - actual).abs();
    let delta: f64 = 0.01;
    if diff > delta {
        panic!("Actual {actual} differs from expected {expected} by more than {delta}");
    }
}
