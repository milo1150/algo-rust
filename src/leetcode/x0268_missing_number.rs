use std::collections::HashMap;

#[allow(dead_code)]
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let min = 0;
    let max = nums.len();

    let mut stack = (min..=max).fold(HashMap::new(), |mut acc, value| {
        acc.insert(value, 0);
        return acc;
    });

    nums.iter().for_each(|&v| {
        stack.entry(v as usize).and_modify(|v| *v += 1);
    });

    let result = stack.iter().find(|(_, &value)| value == 0).unwrap().0;

    *result as i32
}

#[allow(dead_code)]
pub fn missing_number_v1(nums: Vec<i32>) -> i32 {
    let min = 0_usize;
    let max = nums.len();
    for num in min..=max {
        let n = num as i32;
        let check = nums.contains(&n);
        if !check {
            return n;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn missing_number_1() {
        assert_eq!(missing_number(vec![3, 0, 1]), 2)
    }

    #[test]
    #[ignore]
    fn missing_number_2() {
        assert_eq!(missing_number(vec![0, 1]), 2)
    }

    #[test]
    #[ignore]
    fn missing_number_3() {
        assert_eq!(missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]), 8)
    }

    #[test]
    #[ignore]
    fn missing_number_4() {
        assert_eq!(missing_number(vec![1]), 0)
    }

    #[test]
    #[ignore]
    fn missing_number_5() {
        assert_eq!(missing_number(vec![0]), 1)
    }
}
