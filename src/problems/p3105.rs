//! LeetCode 第3105题：最长的严格递增或递减子数组
//!
//! 题目链接：https://leetcode.cn/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
//!
//! 给你一个整数数组 nums。
//!
//! 返回数组 nums 中 严格递增 或 严格递减 的最长非空子数组的长度。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [1,4,3,3,2]
//! - 输出：2
//! - 解释：nums 中严格递增的子数组有 [1]、[2]、[3]、[3]、[4] 以及 [1,4]。
//!   nums 中严格递减的子数组有 [1]、[2]、[3]、[3]、[4]、[3,2] 以及 [4,3]。
//!   因此，返回 2。
//!
//! 示例 2：
//! - 输入：nums = [3,3,3,3]
//! - 输出：1
//! - 解释：nums 中严格递增的子数组有 [3]、[3]、[3] 以及 [3]。
//!   nums 中严格递减的子数组有 [3]、[3]、[3] 以及 [3]。
//!   因此，返回 1。
//!
//! 示例 3：
//! - 输入：nums = [3,2,1]
//! - 输出：3
//! - 解释：nums 中严格递增的子数组有 [3]、[2] 以及 [1]。
//!   nums 中严格递减的子数组有 [3]、[2]、[1]、[3,2]、[2,1] 以及 [3,2,1]。
//!   因此，返回 3。
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 50
//! - 1 <= nums[i] <= 50

use crate::Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut r1 = 1;
        let mut r2 = 1;
        let mut res = 1;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                r1 += 1;
                r2 = 1;
            } else if nums[i] < nums[i - 1] {
                r2 += 1;
                r1 = 1;
            } else {
                r1 = 1;
                r2 = 1;
            }

            res = res.max(r1);
            res = res.max(r2);
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::longest_monotonic_subarray(vec![3, 2, 1]), 3);
    }
}
