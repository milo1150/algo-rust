use std::collections::HashMap;

/// https://leetcode.com/problems/isomorphic-strings/
#[allow(dead_code)]
pub fn is_isomorphic(s: String, t: String) -> bool {
    let mut mapping: HashMap<String, String> = HashMap::new();

    for (index, chr_a) in s.chars().enumerate() {
        let chr_b = t.chars().nth(index).unwrap();
        let check = mapping.get(&chr_a.to_string());
        let check_is_has_b_in = mapping
            .iter()
            .find(|v| *v.1.to_string() == chr_b.to_string());

        if let Some(b) = check_is_has_b_in {
            if *b.0 != chr_a.to_string() || *b.1 != chr_b.to_string() {
                return false;
            }
        }

        if let Some(chr_a_val) = check {
            if *chr_a_val != chr_b.to_string() {
                return false;
            }
        } else {
            mapping
                .entry(chr_a.to_string())
                .or_insert(chr_b.to_string());
        }
    }

    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn is_isomorphic_1() {
        assert_eq!(is_isomorphic(format!("egg"), format!("add")), true)
    }

    #[test]
    #[ignore]
    fn is_isomorphic_2() {
        assert_eq!(is_isomorphic(format!("foo"), format!("bar")), false)
    }

    #[test]
    #[ignore]
    fn is_isomorphic_3() {
        assert_eq!(is_isomorphic(format!("paper"), format!("title")), true)
    }

    #[test]
    #[ignore]
    fn is_isomorphic_4() {
        assert_eq!(is_isomorphic(format!("badc"), format!("baba")), false)
    }
}
