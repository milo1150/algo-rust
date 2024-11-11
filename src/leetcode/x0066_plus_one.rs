/// https://leetcode.com/problems/plus-one/
#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut is_add: bool = true;
    let mut result: Vec<i32> = vec![];

    for value in digits.into_iter().rev() {
        if value == 9 && is_add {
            result.insert(0, 0);
        }

        if value <= 9 && !is_add {
            result.insert(0, value);
        }

        if value < 9 && is_add {
            result.insert(0, value + 1);
            is_add = false;
        }
    }

    if is_add {
        result.insert(0, 1);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn plus_one_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    #[ignore]
    fn plus_one_2() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    #[ignore]
    fn plus_one_3() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }

    #[test]
    #[ignore]
    fn plus_one_4() {
        assert_eq!(
            plus_one(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]),
            vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1]
        );
    }

    #[test]
    #[ignore]
    fn plus_one_5() {
        assert_eq!(
            plus_one(vec![
                7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9,
                4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 6
            ]),
            vec![
                7, 2, 8, 5, 0, 9, 1, 2, 9, 5, 3, 6, 6, 7, 3, 2, 8, 4, 3, 7, 9, 5, 7, 7, 4, 7, 4, 9,
                4, 7, 0, 1, 1, 1, 7, 4, 0, 0, 7
            ]
        );
    }

    #[test]
    #[ignore]
    fn plus_one_6() {
        assert_eq!(plus_one(vec![9, 9]), vec![1, 0, 0]);
    }

    #[test]
    #[ignore]
    fn plus_one_7() {
        assert_eq!(plus_one(vec![8, 9, 9, 9]), vec![9, 0, 0, 0]);
    }

    #[test]
    #[ignore]
    fn plus_one_8() {
        assert_eq!(plus_one(vec![8, 7, 6, 9]), vec![8, 7, 7, 0]);
    }

    #[test]
    #[ignore]
    fn plus_one_9() {
        assert_eq!(plus_one(vec![8, 9, 9, 3]), vec![8, 9, 9, 4]);
    }
}
