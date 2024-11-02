/// https://leetcode.com/problems/palindrome-number/description/
#[allow(dead_code)]
pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || x <= i32::MIN || x > i32::MAX {
        return false;
    }
    let revert_str: String = x.to_string().chars().rev().collect();
    return x.to_string() == revert_str;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn is_palindrome_1() {
        assert_eq!(is_palindrome(121), true);
    }

    #[test]
    #[ignore]
    fn is_palindrome_2() {
        assert_eq!(is_palindrome(-121), false);
    }

    #[test]
    #[ignore]
    fn is_palindrome_3() {
        assert_eq!(is_palindrome(10), false);
    }
}
