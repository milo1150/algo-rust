#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut current_num = nums[0];
    let mut stack: Vec<i32> = vec![nums[0]];

    for num in nums.iter_mut() {
        if *num != current_num {
            stack.push(*num);
            current_num = *num;
        }
    }

    *nums = stack;

    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn remove_duplicates_1() {
        assert_eq!(remove_duplicates(&mut vec![1, 1, 2]), 2);
    }

    #[test]
    #[ignore]
    fn remove_duplicates_2() {
        assert_eq!(
            remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
            5
        );
    }
}
