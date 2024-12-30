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

#[test]
fn age_on_mercury() {
    let age_in_seconds = 2_134_835_688;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Mercury::years_during(&duration);
    let expected = 280.88;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_venus() {
    let age_in_seconds = 189_839_836;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Venus::years_during(&duration);
    let expected = 9.78;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_mars() {
    let age_in_seconds = 2_329_871_239;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Mars::years_during(&duration);
    let expected = 39.25;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_jupiter() {
    let age_in_seconds = 901_876_382;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Jupiter::years_during(&duration);
    let expected = 2.41;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_saturn() {
    let age_in_seconds = 3_000_000_000;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Saturn::years_during(&duration);
    let expected = 3.23;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_uranus() {
    let age_in_seconds = 3_210_123_456;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Uranus::years_during(&duration);
    let expected = 1.21;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_neptune() {
    let age_in_seconds = 1_821_023_456;
    let duration = exercism::Duration::from(age_in_seconds);

    let actual = exercism::Neptune::years_during(&duration);
    let expected = 0.35;

    common::assert_approx_equal_numbers(expected, actual);
}
