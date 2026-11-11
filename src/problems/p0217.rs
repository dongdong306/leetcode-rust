//! LeetCode 第217题：存在重复元素
//!
//! 题目链接：https://leetcode.cn/problems/contains-duplicate/
//!
//! 给你一个整数数组 nums 。如果任一值在数组中出现 至少两次 ，返回 true ；如果数组中每个元素互不相同，返回 false 。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [1,2,3,1]
//! - 输出：true
//!
//! 示例 2：
//! - 输入：nums = [1,2,3,4]
//! - 输出：false
//!
//! 示例 3：
//! - 输入：nums = [1,1,1,3,3,4,3,2,4,2]
//! - 输出：true
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 10^5
//! - -10^9 <= nums[i] <= 10^9

use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for n in nums.iter() {
            if !set.insert(n) {
                return true;
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_example_3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }
}
