/// https://leetcode.com/problems/merge-sorted-array
#[allow(dead_code)]
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, _n: i32) {
    nums1.truncate(m as usize);
    nums1.extend(nums2.iter());
    nums1.sort();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn merge_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let m = 3;
        let mut nums2 = vec![2, 5, 6];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    #[ignore]
    fn merge_2() {
        let mut nums1 = vec![-1, 0, 0, 3, 3, 3, 0, 0, 0];
        let m = 6;
        let mut nums2 = vec![1, 2, 2];
        let n = 3;
        merge(&mut nums1, m, &mut nums2, n);
        assert_eq!(nums1, vec![-1, 0, 0, 1, 2, 2, 3, 3, 3]);
    }
}
