mod common;
use exercism::*;

#[test]
fn age_on_earth() {
    let age_in_seconds = 1_000_000_000;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Earth::years_during(&duration);
    let expected = 31.69;

    common::assert_approx_equal_numbers(expected, actual);
}
