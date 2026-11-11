//! LeetCode 第26题：删除有序数组中的重复项
//!
//! 题目链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-array/
//!
//! 给你一个 非严格递增排列 的数组 nums ，请你 原地 删除重复出现的元素，使每个元素 只出现一次 ，返回删除后数组的新长度。元素的 相对顺序 应该保持 一致 。
//!
//! 考虑 nums 的唯一元素的数量为 k。去重后，返回唯一元素的数量 k。
//!
//! nums 的前 k 个元素应包含 排序后 的唯一数字。下标 k - 1 之后的剩余元素可以忽略。
//!
//! ## 判题标准
//!
//! 系统会用下面的代码来测试你的题解:
//! ```text
//! int[] nums = [...]; // 输入数组
//! int[] expectedNums = [...]; // 长度正确的期望答案
//!
//! int k = removeDuplicates(nums); // 调用
//!
//! assert k == expectedNums.length;
//! for (int i = 0; i < k; i++) {
//!     assert nums[i] == expectedNums[i];
//! }
//! ```
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [1,1,2]
//! - 输出：2, nums = [1,2,_]
//! - 解释：函数应该返回新的长度 2 ，并且原数组 nums 的前两个元素被修改为 1, 2。不需要考虑数组中超出新长度后面的元素。
//!
//! 示例 2：
//! - 输入：nums = [0,0,1,1,1,2,2,3,3,4]
//! - 输出：5, nums = [0,1,2,3,4,_,_,_,_,_]
//! - 解释：函数应该返回新的长度 5 ， 并且原数组 nums 的前五个元素被修改为 0, 1, 2, 3, 和 4 。不需要考虑数组中超出新长度后面的元素。
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 3 * 10^4
//! - -100 <= nums[i] <= 100
//! - nums 已按 非递减 顺序排列。

use crate::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 || nums.len() == 1 {
            return nums.len() as i32;
        }

        for i in (1..nums.len()).rev() {
            if nums[i - 1] == nums[i] {
                nums.remove(i);
            }
        }

        return nums.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut nums = vec![1, 1, 2];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(&nums[..k as usize], &[1, 2]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], &[0, 1, 2, 3, 4]);
    }
}

