use std::collections::HashMap;

/// https://leetcode.com/problems/contains-duplicate-ii/
#[allow(dead_code)]
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut stack: HashMap<i32, i32> = HashMap::new();
    let result: bool = false;

    for (current_index, value) in nums.iter().enumerate() {
        if let Some(prev_seen) = stack.get(&value) {
            let diff = (*prev_seen - current_index as i32).abs();
            let check = diff <= k;
            if check {
                return true;
            } else {
                stack
                    .entry(*value)
                    .and_modify(|v| *v = current_index as i32);
            }
        } else {
            stack.entry(*value).or_insert(current_index as i32);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn contains_nearby_duplicate_1() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1], 3), true)
    }

    #[test]
    #[ignore]
    fn contains_nearby_duplicate_2() {
        assert_eq!(contains_nearby_duplicate(vec![1, 0, 1, 1], 1), true)
    }

    #[test]
    #[ignore]
    fn contains_nearby_duplicate_3() {
        assert_eq!(contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2), false)
    }

    #[test]
    #[ignore]
    fn contains_nearby_duplicate_4() {
        assert_eq!(contains_nearby_duplicate(vec![99, 99], 2), true)
    }

    #[test]
    #[ignore]
    fn contains_nearby_duplicate_5() {
        assert_eq!(contains_nearby_duplicate(vec![1], 1), false)
    }

    #[test]
    #[ignore]
    fn contains_nearby_duplicate_6() {
        assert_eq!(contains_nearby_duplicate(vec![0, 1, 2, 3, 2, 5], 3), true)
    }
}
