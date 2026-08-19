//! LeetCode 第169题：多数元素
//!
//! 题目链接：https://leetcode.cn/problems/majority-element/
//!
//! 给定一个大小为 n 的数组 nums ，返回其中的多数元素。
//! 多数元素是指在数组中出现次数 大于 ⌊ n/2 ⌋ 的元素。
//!
//! 你可以假设数组是非空的，并且给定的数组总是存在多数元素。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [3,2,3]
//! - 输出：3
//!
//! 示例 2：
//! - 输入：nums = [2,2,1,1,1,2,2]
//! - 输出：2
//!
//! ## 提示
//!
//! - n == nums.length
//! - 1 <= n <= 5 * 10^4
//! - -10^9 <= nums[i] <= 10^9
//! - 输入保证数组中一定有一个多数元素。
//!
//! ## 进阶
//!
//! 尝试设计时间复杂度为 O(n)、空间复杂度为 O(1) 的算法解决此问题。

use std::collections::HashMap;
use crate::Solution;

impl Solution {
    /// Boyer-Moore 摩尔投票算法：O(n) 时间，O(1) 空间。
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();

        for num in &nums {
            map.insert(num, map.get(&num).unwrap_or(&0) + 1);
        }

        let n = nums.len();
        for entry in map {
            if entry.1 > n / 2 {
                return *entry.0
            }
        }

        return 0
    }
}
