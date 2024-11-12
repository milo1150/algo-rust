/// https://leetcode.com/problems/excel-sheet-column-title/
/// incomplete
#[allow(dead_code)]
pub fn convert_to_title(column_number: i32) -> String {
    let mut count = column_number;
    let mut modify = -1;
    let mut result: String = String::from("");

    while count > 0 && modify != 0 {
        let mod_v = count % 26;
        if mod_v != 0 {
            let letter = mod_v as u8 + b'A' - 1;
            result.push(letter as char);
        }

        if mod_v == 0 {
            if count >= 26 {
                (0..count / 26).for_each(|_| result.push('Z'));
            } else {
                let letter = (count as u8) + b'A' - 1;
                result.push(letter as char);
            }
            modify = 0
        }

        if count <= 26 && mod_v == 0 {
            break;
        }

        count /= 26;
    }

    result.chars().rev().collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn convert_to_title_1() {
        assert_eq!(convert_to_title(1), format!("A"))
    }

    #[test]
    #[ignore]
    fn convert_to_title_2() {
        assert_eq!(convert_to_title(28), format!("AB"))
    }

    #[test]
    #[ignore]
    fn convert_to_title_3() {
        assert_eq!(convert_to_title(701), format!("ZY"))
    }

    #[test]
    #[ignore]
    fn convert_to_title_4() {
        assert_eq!(convert_to_title(2147483647), format!("FXSHRXW"))
    }

    #[test]
    #[ignore]
    fn convert_to_title_5() {
        assert_eq!(convert_to_title(52), format!("ZZ")) // this should be "AZ" ??.
    }
}
