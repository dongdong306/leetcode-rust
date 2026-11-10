//! LeetCode 第3736题：最小操作次数使数组元素相等 III
//!
//! 题目链接：https://leetcode.cn/problems/minimum-moves-to-equal-array-elements-iii/
//!
//! 给你一个整数数组 nums。
//!
//! 在一步操作中，你可以将任意单个元素 nums[i] 的值增加 1。
//!
//! 返回使数组中的所有元素都相等所需的最小总操作次数。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [2,1,3]
//! - 输出：3
//! - 解释：将 nums[0] 增加到 3（1 次），将 nums[1] 依次增加到 3（2 次），
//!   所有元素都等于 3，最小总操作次数为 3。
//!
//! 示例 2：
//! - 输入：nums = [4,4,5]
//! - 输出：2
//! - 解释：将两个 4 各增加 1 变成 5，最小总操作次数为 2。
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 100
//! - 1 <= nums[i] <= 100

use crate::Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut r = 0;

        for i in 0..nums.len() {
            if m < nums[i] {
                r += (i as i32) * (nums[i] - m);
                m = nums[i];
            } else {
                r += m - nums[i];
            }
        }

        return r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::min_moves(vec![2, 1, 3]), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::min_moves(vec![4, 4, 5]), 2);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(Solution::min_moves(vec![7]), 0);
    }
}
