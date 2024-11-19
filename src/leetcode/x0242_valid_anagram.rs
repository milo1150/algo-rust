use std::collections::HashMap;

#[allow(dead_code)]
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_count: HashMap<char, i32> = HashMap::new();
    let mut t_count: HashMap<char, i32> = HashMap::new();
    let mut result: bool = true;

    for chr in s.chars() {
        s_count.entry(chr).and_modify(|v| *v += 1).or_insert(1);
    }

    for chr in t.chars() {
        t_count.entry(chr).and_modify(|v| *v += 1).or_insert(1);
    }

    for (&chr, &count) in s_count.iter() {
        if let Some(t_count) = t_count.get(&chr) {
            if count != *t_count {
                result = false;
                break;
            }
        } else {
            result = false
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn is_anagram_1() {
        assert_eq!(is_anagram(format!("anagram"), format!("nagaram")), true)
    }

    #[test]
    #[ignore]
    fn is_anagram_2() {
        assert_eq!(is_anagram(format!("rat"), format!("car")), false)
    }
}
