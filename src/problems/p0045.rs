//! LeetCode 第45题：跳跃游戏 II
//!
//! 题目链接：https://leetcode.cn/problems/jump-game-ii/
//!
//! 给定一个长度为 n 的 0 索引整数数组 nums。初始位置为 nums[0]。
//!
//! 每个元素 nums[i] 表示从索引 i 向前跳转的最大长度。换句话说，如果你在 nums[i] 处，你可以跳转到任意 nums[i + j] 处：
//!
//! - 0 <= j <= nums[i]
//! - i + j < n
//!
//! 返回到达 nums[n - 1] 的最小跳跃次数。生成的测试用例可以到达 nums[n - 1]。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [2,3,1,1,4]
//! - 输出：2
//! - 解释：跳到最后一个位置的最小跳跃数是 2。从下标 0 跳到下标 1，跳 1 步，然后跳 3 步到达数组的最后一个位置。
//!
//! 示例 2：
//! - 输入：nums = [2,3,0,1,4]
//! - 输出：2
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 10^4
//! - 0 <= nums[i] <= 1000
//! - 保证可以到达 nums[n-1]
//!
//! ## 进阶
//!
//! 你能设计一个 O(n) 时间复杂度、O(1) 空间复杂度的算法解决此问题吗？

use std::cmp::{max, min};
use crate::Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![i32::MAX; nums.len()];
        dp[0] = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::jump(vec![1]), 0);
    }
}
