use std::collections::HashMap;

#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut stack: HashMap<String, i32> = HashMap::new();

    for v in nums {
        stack
            .entry(v.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    let max = stack
        .iter()
        .max_by(|current_max, next| current_max.1.cmp(next.1))
        .unwrap();

    max.0.parse().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn majority_element_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3)
    }

    #[test]
    #[ignore]
    fn majority_element_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2)
    }
}
