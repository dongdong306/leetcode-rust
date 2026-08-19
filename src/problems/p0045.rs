use std::cmp::{max, min};
use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; nums.len()];
        for (i, num) in nums.iter().enumerate() {
            for j in 1 .. nums[i] + 1 {
                if (i + j as usize) < nums.len() {
                    dp[i + j as usize] = min(dp[i] + 1, dp[i + j as usize])
                }
            }
        }

        return dp[nums.len()-1];
    }
}
