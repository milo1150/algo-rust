use std::collections::HashMap;

#[allow(dead_code)]
pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    let mut nums2_count = nums2
        .clone()
        .into_iter()
        .fold(HashMap::new(), |mut acc, cur| {
            acc.entry(cur).and_modify(|v| *v += 1).or_insert(1);
            return acc;
        });

    for num in nums1.iter() {
        let is_contain_num = nums2_count.get(num);
        if let Some(count) = is_contain_num {
            if *count != 0 {
                result.push(*num);
                nums2_count.entry(*num).and_modify(|v| *v -= 1);
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn intersect_1() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2, 2]), vec![2, 2])
    }

    #[test]
    #[ignore]
    fn intersect_2() {
        assert_eq!(intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]), vec![4, 9])
    }

    #[test]
    #[ignore]
    fn intersect_3() {
        assert_eq!(intersect(vec![1, 2, 2, 1], vec![2]), vec![2])
    }

    #[test]
    #[ignore]
    fn intersect_4() {
        assert_eq!(intersect(vec![2, 1], vec![1, 2]), vec![2, 1])
    }
}
