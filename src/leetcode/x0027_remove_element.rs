/// https://leetcode.com/problems/remove-element/
#[allow(dead_code)]
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|v| *v != val);
    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn remove_element_1() {
        assert_eq!(remove_element(&mut vec![3, 2, 2, 3], 3), 2);
    }

    #[test]
    #[ignore]
    fn remove_element_2() {
        assert_eq!(remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2), 5);
    }
}
