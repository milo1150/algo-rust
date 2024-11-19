#[allow(dead_code)]
pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() {
        return vec![];
    }

    let mut result: Vec<String> = vec![];
    let mut current_index = 0;

    while current_index <= nums.len() {
        let current_num = nums.get(current_index).unwrap();

        if current_index == nums.len() - 1 {
            result.push(format!("{current_num}"));
            break;
        }

        let mut acc = current_num;
        let mut next_index = 0;

        for current_j in current_index + 1..=nums.len() {
            let next = nums.get(current_j).unwrap();
            let is_not_sort = acc + 1 != *next;

            if !is_not_sort && current_j == nums.len() - 1 {
                result.push(format!("{current_num:?}->{next:?}"));
                next_index = nums.len() + 1;
                break;
            }

            if is_not_sort {
                if current_num == acc {
                    result.push(format!("{current_num:?}"));
                } else {
                    result.push(format!("{current_num:?}->{acc:?}"));
                }

                next_index = current_j;
                break;
            }

            if acc + 1 == *next {
                acc = next;
            }
        }

        current_index = next_index;

        if next_index > nums.len() {
            break;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn summary_ranges_1() {
        assert_eq!(
            summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        )
    }

    #[test]
    #[ignore]
    fn summary_ranges_2() {
        assert_eq!(
            summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        )
    }
}
