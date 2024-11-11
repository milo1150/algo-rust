/// https://leetcode.com/problems/search-insert-position/description/
#[allow(dead_code)]
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    for (index, value) in nums.iter().enumerate() {
        if target <= *value {
            return index as i32;
        }
    }

    nums.len() as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn search_insert_1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    #[ignore]
    fn search_insert_2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    #[ignore]
    fn search_insert_3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
