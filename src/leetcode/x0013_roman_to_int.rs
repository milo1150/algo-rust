#[derive(Debug)]
enum RomanNumeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanNumeral {
    fn validate(c: &str) -> Option<RomanNumeral> {
        match c {
            "I" => Some(RomanNumeral::I),
            "V" => Some(RomanNumeral::V),
            "X" => Some(RomanNumeral::X),
            "L" => Some(RomanNumeral::L),
            "C" => Some(RomanNumeral::C),
            "D" => Some(RomanNumeral::D),
            "M" => Some(RomanNumeral::M),
            _ => None,
        }
    }
}

/// https://leetcode.com/problems/roman-to-integer/description/
#[allow(dead_code)]
pub fn roman_to_int(s: String) -> i32 {
    if s.len() < 1 || s.len() > 15 {
        return 0;
    }

    let mut result: i32 = 0;
    let char_list: Vec<char> = s.chars().rev().collect();

    for (index, roman_str) in char_list.iter().enumerate() {
        let roman_numeral = RomanNumeral::validate(&roman_str.to_string());
        match roman_numeral {
            Some(roman_numeral_str) => {
                let prev_char = if index != 0 {
                    char_list.get(index - 1)
                } else {
                    None
                };

                match (roman_numeral_str, prev_char) {
                    (RomanNumeral::I, Some(prev)) => {
                        if let Some(prev_char) = RomanNumeral::validate(&prev.to_string()) {
                            match prev_char {
                                RomanNumeral::V | RomanNumeral::X => result -= 1,
                                _ => result += 1,
                            }
                        }
                    }
                    (RomanNumeral::I, None) => result += 1,
                    (RomanNumeral::V, Some(_)) | (RomanNumeral::V, None) => result += 5,
                    (RomanNumeral::X, Some(prev)) => {
                        if let Some(prev_char) = RomanNumeral::validate(&prev.to_string()) {
                            match prev_char {
                                RomanNumeral::L | RomanNumeral::C => result -= 10,
                                _ => result += 10,
                            }
                        }
                    }
                    (RomanNumeral::X, None) => result += 10,
                    (RomanNumeral::L, Some(_)) | (RomanNumeral::L, None) => result += 50,
                    (RomanNumeral::C, Some(prev)) => {
                        if let Some(prev_char) = RomanNumeral::validate(&prev.to_string()) {
                            match prev_char {
                                RomanNumeral::D | RomanNumeral::M => result -= 100,
                                _ => result += 100,
                            }
                        }
                    }
                    (RomanNumeral::C, None) => result += 100,
                    (RomanNumeral::D, Some(_)) | (RomanNumeral::D, None) => result += 500,
                    (RomanNumeral::M, Some(_)) | (RomanNumeral::M, None) => result += 1000,
                }
            }
            None => return 0,
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn roman_to_int_1() {
        assert_eq!(roman_to_int(String::from("III")), 3);
    }

    #[test]
    #[ignore]
    fn roman_to_int_2() {
        assert_eq!(roman_to_int(String::from("LVIII")), 58);
    }

    #[test]
    #[ignore]
    fn roman_to_int_3() {
        assert_eq!(roman_to_int(String::from("MCMXCIV")), 1994);
    }
}
