use std::cmp::max;
use crate::Solution;

impl Solution {
    pub fn max_profit_ii(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut matrix: Vec<[i32; 2]> = vec![[0; 2]; n];

        matrix[0][1] = -prices[0];

        for i in 1..n {
            matrix[i][0] = max(matrix[i - 1][1] - prices[i], matrix[i - 1][0]);
            matrix[i][1] = max(matrix[i - 1][0] + prices[i], matrix[i - 1][1]);
        }

        return matrix[n-1][0];
    }
}
