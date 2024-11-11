use std::collections::HashMap;

/// https://leetcode.com/problems/longest-common-prefix/
#[allow(dead_code)]
pub fn longest_common_prefix_v1(strs: Vec<String>) -> String {
    if strs.len() < 1 || strs.len() > 200 {
        return format!("");
    }

    let mut result: String = String::from("");
    let mut count = 0;

    while count < strs.first().unwrap().len() as i32 && count >= 0 {
        let mut acc: HashMap<String, i32> = HashMap::new();
        let mut str_byte: Option<&u8> = None;

        for word in strs.iter() {
            let str_byte_at_index = word.as_bytes().get(count as usize);
            str_byte = str_byte_at_index;

            if let Some(single_str_byte) = str_byte_at_index {
                if let Some(str_byte_acc) = acc.get(&single_str_byte.to_string()) {
                    let stack = str_byte_acc + 1;
                    acc.insert(single_str_byte.to_string(), stack);
                } else {
                    acc.insert(single_str_byte.to_string(), 1);
                }
            } else {
                count = -1;
                break;
            }
        }

        if count < 0 || acc.len() > 1 {
            break;
        } else {
            let single_str = char::from(*str_byte.unwrap());
            result.push(single_str);
            count += 1;
        }
    }

    result
}

#[allow(dead_code)]
pub fn longest_common_prefix_v2(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }

    let mut result = String::from("");
    let min_length = strs.iter().map(|v| v.len()).min().unwrap_or(0);

    for index in 0..min_length {
        let char_at_indexs: Vec<char> = strs.iter().filter_map(|w| w.chars().nth(index)).collect();
        let is_all_same_char: bool = char_at_indexs.iter().all(|v| *v == char_at_indexs[0]);
        if is_all_same_char {
            result.push(char_at_indexs[0]);
        } else {
            break;
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn longest_common_prefix_1() {
        assert_eq!(
            longest_common_prefix_v1(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            "fl"
        )
    }

    #[test]
    #[ignore]
    fn longest_common_prefix_2() {
        assert_eq!(
            longest_common_prefix_v1(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            ""
        )
    }

    #[test]
    #[ignore]
    fn longest_common_prefix_fp_1() {
        assert_eq!(
            longest_common_prefix_v2(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            "fl"
        )
    }

    #[test]
    #[ignore]
    fn longest_common_prefix_fp_2() {
        assert_eq!(
            longest_common_prefix_v2(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            ""
        )
    }
}
