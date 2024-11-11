use std::collections::HashMap;

/// https://leetcode.com/problems/valid-parentheses/
#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    if s.is_empty() || s.len() % 2 != 0 {
        return false;
    }

    let mut stack: Vec<char> = vec![];
    let map_bracket: HashMap<char, char> =
        [('}', '{'), (']', '['), (')', '(')].into_iter().collect();

    for ch in s.chars() {
        match ch {
            '{' | '[' | '(' => stack.push(ch),
            '}' | ']' | ')' => {
                let prev_ch = stack.last();
                let Some(v) = prev_ch else {
                    return false;
                };
                let map_value = map_bracket.get(&ch).unwrap();

                if v == map_value {
                    stack.pop();
                } else {
                    break;
                }
            }
            _ => unreachable!(),
        }
    }

    stack.len() == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn is_valid_1() {
        assert_eq!(is_valid(String::from("()")), true);
    }

    #[test]
    #[ignore]
    fn is_valid_2() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }

    #[test]
    #[ignore]
    fn is_valid_3() {
        assert_eq!(is_valid(String::from("(]")), false);
    }

    #[test]
    #[ignore]
    fn is_valid_4() {
        assert_eq!(is_valid(String::from("([])")), true);
    }

    #[test]
    #[ignore]
    fn is_valid_5() {
        assert_eq!(is_valid(String::from("([)]")), false);
    }

    #[test]
    #[ignore]
    fn is_valid_6() {
        assert_eq!(is_valid(String::from("([{}])()")), true);
    }

    #[test]
    #[ignore]
    fn is_valid_7() {
        assert_eq!(is_valid(String::from("([{]})()")), false);
    }

    #[test]
    #[ignore]
    fn is_valid_8() {
        assert_eq!(is_valid(String::from("(((())))[[{}]]")), true);
    }

    #[test]
    #[ignore]
    fn is_valid_9() {
        assert_eq!(is_valid(String::from("){")), false);
    }
}
