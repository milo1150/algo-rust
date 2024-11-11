#[allow(dead_code)]
pub fn is_palindrome(s: String) -> bool {
    let clean_text: String = s
        .chars()
        .filter(|c| c.is_alphabetic() || c.is_numeric())
        .flat_map(|c| c.to_lowercase())
        .collect();
    let revert_clean_text: String = clean_text.chars().rev().collect();
    clean_text == revert_clean_text
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_palindrome_1() {
        assert_eq!(
            is_palindrome(format!("A man, a plan, a canal: Panama")),
            true
        )
    }

    #[test]
    fn is_palindrome_2() {
        assert_eq!(is_palindrome(format!("race a car")), false)
    }

    #[test]
    fn is_palindrome_3() {
        assert_eq!(is_palindrome(format!(" ")), true)
    }

    #[test]
    fn is_palindrome_4() {
        assert_eq!(is_palindrome(format!("0P")), false)
    }
}
