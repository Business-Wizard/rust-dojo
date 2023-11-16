#![allow(unused_variables, dead_code, unused_mut, unused_imports)]

pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits = Vec::new();
    num.to_string()
        .chars()
        .for_each(|char| digits.push(char.to_digit(10).unwrap()));
    let length = digits.len();
    let maybe_armstrong: u32 = digits.iter().map(|digit| digit.pow(length as u32)).sum();
    maybe_armstrong == num
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
}
