//! LeetCode 第88题：合并两个有序数组
//!
//! 题目链接：https://leetcode.cn/problems/merge-sorted-array/
//!
//! 给你两个按 非递减顺序 排列的整数数组 nums1 和 nums2，另有两个整数 m 和 n ，分别表示 nums1 和 nums2 中的元素数目。
//!
//! 请你 合并 nums2 到 nums1 中，使合并后的数组同样按 非递减顺序 排列。
//!
//! 注意：最终，合并后数组不应由函数返回，而是存储在数组 nums1 中。为了应对这种情况，nums1 的初始长度为 m + n，其中前 m 个元素表示应合并的元素，后 n 个元素为 0 ，应忽略。nums2 的长度为 n 。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
//! - 输出：nums1 = [1,2,2,3,5,6]
//! - 解释：需要合并 [1,2,3] 和 [2,5,6] 。合并结果是 [1,2,2,3,5,6] 。
//!
//! 示例 2：
//! - 输入：nums1 = [1], m = 1, nums2 = [], n = 0
//! - 输出：nums1 = [1]
//! - 解释：需要合并 [1] 和 [] 。合并结果是 [1] 。
//!
//! 示例 3：
//! - 输入：nums1 = [0], m = 0, nums2 = [1], n = 1
//! - 输出：nums1 = [1]
//! - 解释：需要合并的数组是 [] 和 [1] 。合并结果是 [1] 。注意，因为 m = 0 ，所以 nums1 中没有元素。nums1 中仅存的 0 仅仅是为了确保合并结果可以顺利存放到 nums1 中。
//!
//! ## 提示
//!
//! - nums1.length == m + n
//! - nums2.length == n
//! - 0 <= m, n <= 200
//! - 1 <= m + n <= 200
//! - -10^9 <= nums1[i], nums2[j] <= 10^9
//!
//! ## 进阶
//!
//! 你可以设计实现一个时间复杂度为 O(m + n) 的算法解决此问题吗？

use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut j = m as usize;
        let mut k = n as usize;
        for i in (0 .. nums1.len()).rev() {
            if j == 0 || k == 0 {
                break;
            }

            let a = nums1[j - 1];
            let b = nums2[k - 1];

            if a > b {
                nums1[i] = a;
                j = j - 1;
            } else {
                nums1[i] = b;
                k = k -1;
            }

        }

        if m == 0 {
            for i in 0 .. nums2.len() {
                nums1[i] = nums2[i];
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }

    #[test]
    fn test_example_2() {
        let mut nums1 = vec![1];
        let mut nums2: Vec<i32> = vec![];
        Solution::merge(&mut nums1, 1, &mut nums2, 0);
        assert_eq!(nums1, vec![1]);
    }

    #[test]
    fn test_example_3() {
        let mut nums1 = vec![0];
        let mut nums2 = vec![1];
        Solution::merge(&mut nums1, 0, &mut nums2, 1);
        assert_eq!(nums1, vec![1]);
    }
}
