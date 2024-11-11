#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for current_price in prices {
        min_price = min_price.min(current_price);
        max_profit = max_profit.max(current_price - min_price);
    }

    max_profit
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn max_profit_1() {
        assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    #[ignore]
    fn max_profit_2() {
        assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
    }

    #[test]
    #[ignore]
    fn max_profit_3() {
        assert_eq!(max_profit(vec![1, 2]), 1);
    }

    // #[test]
    // fn max_profit_4() {
    //     assert_eq!(max_profit(vec![3, 3]), 0);
    // }

    // #[test]
    // fn max_profit_5() {
    //     assert_eq!(max_profit(vec![1, 4, 2]), 3);
    // }

    // // Test with a single day (no opportunity for profit)
    // #[test]
    // fn max_profit_single_day() {
    //     assert_eq!(max_profit(vec![5]), 0);
    // }

    // // Test with prices decreasing every day (no profit)
    // #[test]
    // fn max_profit_decreasing() {
    //     assert_eq!(max_profit(vec![5, 4, 3, 2, 1]), 0); // No profit possible
    // }

    // // Test with prices fluctuating, random example
    // #[test]
    // fn max_profit_fluctuating() {
    //     assert_eq!(max_profit(vec![2, 4, 1, 3, 6, 2, 5]), 5); // Buy at 1 (index 2), sell at 6 (index 4)
    // }

    // // Test with the prices already at a high value followed by a drop
    // #[test]
    // fn max_profit_high_then_drop() {
    //     assert_eq!(max_profit(vec![10, 9, 8, 7, 6, 5]), 0); // No profit possible
    // }

    // // Test with prices all equal (no profit possible)
    // #[test]
    // fn max_profit_all_equal() {
    //     assert_eq!(max_profit(vec![5, 5, 5, 5, 5]), 0); // No profit possible
    // }

    // // Test with larger price range with no profit
    // #[test]
    // fn max_profit_no_opportunity() {
    //     assert_eq!(max_profit(vec![5000, 4000, 3000, 2000, 1000]), 0); // No opportunity for profit
    // }
}
