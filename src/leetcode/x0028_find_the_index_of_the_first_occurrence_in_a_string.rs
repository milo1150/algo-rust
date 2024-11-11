#[allow(dead_code)]
pub fn str_str(haystack: String, needle: String) -> i32 {
    if haystack == needle {
        return 0;
    }

    let end = if needle.len() == 1 {
        haystack.len()
    } else {
        haystack.len() - needle.len()
    };

    for index in 0..=end {
        let end = index + &needle.len();
        if end > haystack.len() {
            return -1;
        }
        if &haystack[index..end] == needle {
            return index as i32;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn str_str_1() {
        assert_eq!(str_str(format!("sadbutsad"), format!("sad")), 0);
    }

    #[test]
    #[ignore]
    fn str_str_2() {
        assert_eq!(str_str(format!("leetcode"), format!("leeto")), -1);
    }

    #[test]
    #[ignore]
    fn str_str_3() {
        assert_eq!(str_str(format!("hello"), format!("ll")), 2);
    }

    #[test]
    #[ignore]
    fn str_str_4() {
        assert_eq!(str_str(format!("mississippi"), format!("a")), -1);
    }

    #[test]
    #[ignore]
    fn str_str_5() {
        assert_eq!(str_str(format!("mississippi"), format!("issi")), 1);
    }

    #[test]
    #[ignore]
    fn str_str_6() {
        assert_eq!(str_str(format!("a"), format!("a")), 0);
    }

    #[test]
    #[ignore]
    fn str_str_7() {
        assert_eq!(str_str(format!("abc"), format!("c")), 2);
    }
}
