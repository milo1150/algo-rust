use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
pub fn word_pattern(pattern: String, s: String) -> bool {
    let split_pattern: Vec<char> = pattern.chars().collect();
    let split_s: Vec<&str> = s.split(" ").collect();

    if split_pattern.len() != split_s.len() {
        return false;
    }

    let mut stack: HashMap<char, String> = HashMap::new();

    for index in 0..split_s.len() {
        let key = split_pattern.get(index).unwrap();
        let value = split_s.get(index).unwrap();
        let find_key_value = stack.get_key_value(key);

        if let Some((stack_key, stack_value)) = find_key_value {
            if key == stack_key && value == stack_value {
                continue;
            }
            if key != stack_key && value == stack_value || key == stack_key && value != stack_value
            {
                return false;
            }
        } else {
            stack.insert(*key, value.to_string());
        }
    }

    let check_duplicate = stack.values().collect::<HashSet<_>>();
    if check_duplicate.len() != stack.len() {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn word_pattern_1() {
        assert_eq!(
            word_pattern(format!("abba"), format!("dog cat cat dog")),
            true
        )
    }

    #[test]
    #[ignore]
    fn word_pattern_2() {
        assert_eq!(
            word_pattern(format!("abba"), format!("dog cat cat fish")),
            false
        )
    }

    #[test]
    #[ignore]
    fn word_pattern_3() {
        assert_eq!(
            word_pattern(format!("aaaa"), format!("dog cat cat dog")),
            false
        )
    }

    #[test]
    #[ignore]
    fn word_pattern_4() {
        assert_eq!(
            word_pattern(format!("abba"), format!("dog dog dog dog")),
            false
        )
    }

    #[test]
    #[ignore]
    fn word_pattern_5() {
        assert_eq!(word_pattern(format!("aba"), format!("dog cat cat")), false)
    }
}
