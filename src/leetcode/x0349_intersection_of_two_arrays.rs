use std::collections::HashSet;

#[allow(dead_code)]
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let intersec: Vec<i32> = nums1.into_iter().filter(|v| nums2.contains(v)).collect();
    let set: HashSet<i32> = intersec.into_iter().collect();
    let result: Vec<i32> = set.into_iter().collect();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn intersection_1() {
        assert_eq!(intersection(vec![1, 2, 2, 1], vec![2, 2]), vec![2])
    }

    #[test]
    #[ignore]
    fn intersection_2() {
        assert_eq!(intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9])
    }
}
