use std::collections::HashMap;

#[allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut stack: HashMap<String, i32> = HashMap::new();

    for num in nums {
        stack
            .entry(num.to_string())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    let result = stack.iter().find(|v| *v.1 == 1);

    if let Some(r) = result {
        r.0.parse().unwrap()
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn single_number_1() {
        assert_eq!(single_number(vec![2, 2, 1]), 1)
    }

    #[test]
    #[ignore]
    fn single_number_2() {
        assert_eq!(single_number(vec![4, 1, 2, 1, 2]), 4)
    }
    #[test]
    #[ignore]
    fn single_number_3() {
        assert_eq!(single_number(vec![1]), 1)
    }
}
