#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u128> = num
        .to_string()
        .chars()
        .map(|char| (char.to_digit(10).unwrap() as u128))
        .collect();
    let length = digits.len() as u32;
    let maybe_armstrong: Option<u128> = digits.iter().map(|&digit| digit.checked_pow(length)).sum();
    maybe_armstrong == Some(num as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        (1..=9).all(is_armstrong_number);
    }

    #[test]
    fn double_digit_numbers_are_not_armstrong_numbers() {
        (10..=99).all(|num| !is_armstrong_number(num));
    }

    #[test]
    fn ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }

    #[test]
    fn properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}
