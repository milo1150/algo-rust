use crate::leetcode::x0118_pascal_triangle::generate;

/// https://leetcode.com/problems/pascals-triangle-ii
#[allow(dead_code)]
pub fn get_row(row_index: i32) -> Vec<i32> {
    // V1
    // generate(row_index + 1)
    //     .get(row_index as usize)
    //     .unwrap()
    //     .clone()

    // V2
    generate(row_index + 1).into_iter().last().unwrap_or(vec![])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn ger_row_1() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1])
    }

    #[test]
    #[ignore]
    fn ger_row_2() {
        assert_eq!(get_row(0), vec![1])
    }

    #[test]
    #[ignore]
    fn ger_row_3() {
        assert_eq!(get_row(1), vec![1, 1])
    }
}
