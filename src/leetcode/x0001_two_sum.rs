/// https://leetcode.com/problems/two-sum/description/
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result: [i32; 2] = [0, 0];

    for (index_i, num_i) in nums.iter().take(nums.len() - 1).enumerate() {
        let skip_index = index_i + 1;
        for (index_j, num_j) in nums.iter().skip(skip_index).enumerate() {
            let actual_index_j = index_j + skip_index;
            if num_i + num_j == target {
                result[0] = index_i as i32;
                result[1] = actual_index_j as i32;
            }
        }
    }

    result.to_vec()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn test_two_sum_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    #[ignore]
    fn test_two_sum_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    #[ignore]
    fn test_two_sum_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
