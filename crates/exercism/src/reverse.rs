pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_letters() {
        let input = "onlyletters";
        let actual = reverse(input);
        let expected = "srettelylno";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_with_numbers() {
        let input = "123456";
        let actual = reverse(input);
        let expected = "654321";
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_with_wide_characters() {
        let input = "子猫";
        let actual = reverse(input);
        let expected = "猫子";
        assert_eq!(actual, expected);
    }
}
