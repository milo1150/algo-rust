use std::collections::HashMap;

/// https://leetcode.com/problems/excel-sheet-column-number/
/// incomplete
#[allow(dead_code)]
pub fn title_to_number(column_title: String) -> i32 {
    let alphabet: HashMap<char, i32> = ('A'..='Z').zip(1..=26).collect();

    let mut result: i32 = 0;

    for (index, chr) in column_title.chars().enumerate() {
        let chr_v = *alphabet.get(&chr).unwrap();

        if index == column_title.len() - 1 {
            result += chr_v;
        } else {
            result += chr_v * 26
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn title_to_number_1() {
        assert_eq!(title_to_number(format!("A")), 1)
    }

    #[test]
    #[ignore]
    fn title_to_number_2() {
        assert_eq!(title_to_number(format!("AB")), 28)
    }

    #[test]
    #[ignore]
    fn title_to_number_3() {
        assert_eq!(title_to_number(format!("ZY")), 701)
    }

    #[test]
    #[ignore]
    fn title_to_number_4() {
        assert_eq!(title_to_number(format!("FXSHRXW")), 2147483647)
    }
}
