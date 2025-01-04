mod common;
use exercism::space_age;
use exercism::space_age::*;

#[test]
fn age_on_earth() {
    let age_in_seconds = 1_000_000_000;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Earth::years_during(&duration);
    let expected = 31.69;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_mercury() {
    let age_in_seconds = 2_134_835_688;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Mercury::years_during(&duration);
    let expected = 280.88;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_venus() {
    let age_in_seconds = 189_839_836;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Venus::years_during(&duration);
    let expected = 9.78;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_mars() {
    let age_in_seconds = 2_329_871_239;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Mars::years_during(&duration);
    let expected = 39.25;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_jupiter() {
    let age_in_seconds = 901_876_382;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Jupiter::years_during(&duration);
    let expected = 2.41;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_saturn() {
    let age_in_seconds = 3_000_000_000;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Saturn::years_during(&duration);
    let expected = 3.23;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_uranus() {
    let age_in_seconds = 3_210_123_456;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Uranus::years_during(&duration);
    let expected = 1.21;

    common::assert_approx_equal_numbers(expected, actual);
}

#[test]
fn age_on_neptune() {
    let age_in_seconds = 1_821_023_456;
    let duration = space_age::Duration::from(age_in_seconds);

    let actual = space_age::Neptune::years_during(&duration);
    let expected = 0.35;

    common::assert_approx_equal_numbers(expected, actual);
}
