//! LeetCode 第162题：寻找峰值
//!
//! 题目链接：https://leetcode.cn/problems/find-peak-element/
//!
//! 峰值元素是指其值严格大于左右相邻值的元素。
//!
//! 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个 峰值所在位置即可。
//!
//! 你可以假设 nums[-1] = nums[n] = -∞ 。
//!
//! 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [1,2,3,1]
//! - 输出：2
//! - 解释：3 是峰值元素，你的函数应该返回其索引 2。
//!
//! 示例 2：
//! - 输入：nums = [1,2,1,3,5,6,4]
//! - 输出：1 或 5
//! - 解释：你的函数可以返回索引 1，其峰值元素为 2；或者返回索引 5，其峰值元素为 6。
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 1000
//! - -2^31 <= nums[i] <= 2^31 - 1
//! - 对于所有有效的 i 都有 nums[i] != nums[i + 1]

use crate::Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        let mut m: i32;

        while l < r {
            m = (l + r) / 2;
            if m - 1 >= 0 && nums[m as usize] < nums[(m - 1) as usize] {
                r = m - 1;
            } else if nums[m as usize] < nums[(m + 1) as usize] {
                l = m + 1;
            } else {
                return m;
            }
        }

        return l;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let peak = Solution::find_peak_element(vec![1, 2, 3, 1]);
        assert!(peak == 2, "期望峰值索引 2，实际为 {}", peak);
    }

    #[test]
    fn test_example_2() {
        let peak = Solution::find_peak_element(vec![1, 2, 1, 3, 5, 6, 4]);
        assert!(peak == 1 || peak == 5, "期望峰值索引 1 或 5，实际为 {}", peak);
    }
}
