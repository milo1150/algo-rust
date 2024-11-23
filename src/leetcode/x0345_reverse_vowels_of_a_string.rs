#[allow(dead_code)]
pub fn reverse_vowels(s: String) -> String {
    let vowels: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    let mut result: String = String::from("");
    let mut stack: Vec<char> = vec![];

    for chr in s.chars() {
        let chr_lowercase = chr.to_lowercase().next().unwrap();
        if vowels.contains(&chr_lowercase) {
            stack.push(chr);
        }
    }

    let clone_stack = stack.clone();

    for chr in s.chars() {
        if clone_stack.contains(&chr) {
            let pop = stack.pop().unwrap();
            result.push(pop);
        } else {
            result.push(chr);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn reverse_vowels_1() {
        assert_eq!(reverse_vowels(format!("IceCreAm")), format!("AceCreIm"))
    }

    #[test]
    #[ignore]
    fn reverse_vowels_2() {
        assert_eq!(reverse_vowels(format!("leetcode")), format!("leotcede"))
    }
}
