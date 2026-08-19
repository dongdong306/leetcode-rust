use crate::Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = i32::MAX;
        let mut result = 0;

        for &price in &prices {
            if min > price {
                min = price;
            } else if price - min > result {
                result = price - min;
            }

        }

        return result;
    }
}
