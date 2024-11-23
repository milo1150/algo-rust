use std::collections::HashMap;

#[allow(dead_code)]
pub fn ransome_note(ransom_note: String, magazine: String) -> bool {
    let count_random_note = ransom_note
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, chr| {
            acc.entry(chr).and_modify(|v| *v += 1).or_insert(1);
            return acc;
        });
    let count_magazine = magazine
        .chars()
        .into_iter()
        .fold(HashMap::new(), |mut acc, chr| {
            acc.entry(chr).and_modify(|v| *v += 1).or_insert(1);
            return acc;
        });

    for (key, count) in count_random_note {
        if let Some(v) = count_magazine.get(&key) {
            if *v < count {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    #[ignore]
    fn ransome_note_1() {
        assert_eq!(ransome_note(format!("a"), format!("b")), false)
    }

    #[test]
    #[ignore]
    fn ransome_note_2() {
        assert_eq!(ransome_note(format!("aa"), format!("ab")), false)
    }

    #[test]
    #[ignore]
    fn ransome_note_3() {
        assert_eq!(ransome_note(format!("aa"), format!("aab")), true)
    }

    #[test]
    #[ignore]
    fn ransome_note_4() {
        assert_eq!(
            ransome_note(
                format!("bg"),
                format!("efjbdfbdgfjhhaiigfhbaejahgfbbgbjagbddfgdiaigdadhcfcj")
            ),
            true
        )
    }
}
