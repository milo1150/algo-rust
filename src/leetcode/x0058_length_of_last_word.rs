/// https://leetcode.com/problems/length-of-last-word/description/
#[allow(dead_code)]
pub fn length_of_last_word(s: String) -> i32 {
    let result: i32 = s
        .split_whitespace()
        .last()
        .unwrap()
        .len()
        .try_into()
        .unwrap_or(0);
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn length_of_last_word_1() {
        assert_eq!(length_of_last_word(format!("Hello World")), 5);
    }

    #[test]
    #[ignore]
    fn length_of_last_word_2() {
        assert_eq!(
            length_of_last_word(format!("   fly me   to   the moon  ")),
            4
        );
    }

    #[test]
    #[ignore]
    fn length_of_last_word_3() {
        assert_eq!(length_of_last_word(format!("luffy is still joyboy")), 6);
    }
}
