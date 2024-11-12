use std::collections::HashMap;

/// https://leetcode.com/problems/contains-duplicate/
#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut stack: HashMap<i32, i32> = HashMap::new();
    for val in nums {
        if let Some(_) = stack.get(&val) {
            return true;
        } else {
            stack.entry(val).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn contains_duplicate_1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true)
    }

    #[test]
    #[ignore]
    fn contains_duplicate_2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false)
    }

    #[test]
    #[ignore]
    fn contains_duplicate_3() {
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true)
    }
}
