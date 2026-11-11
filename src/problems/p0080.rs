//! LeetCode 第80题：删除有序数组中的重复项 II
//!
//! 题目链接：https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/
//!
//! 给你一个有序数组 nums ，请你 原地 删除重复出现的元素，使得出现次数超过两次的元素只出现两次 ，返回删除后数组的新长度。
//!
//! 不要使用额外的数组空间，你必须在 原地修改输入数组 并在使用 O(1) 额外空间的条件下完成。
//!
//! ## 说明
//!
//! 为什么返回数值是整数，但输出的答案是数组呢？
//!
//! 请注意，输入数组是以「引用」方式传递的，这意味着在函数里修改输入数组对于调用者是可见的。
//!
//! 你可以想象内部操作如下:
//! ```text
//! // nums 是以“引用”方式传递的。也就是说，不对实参做任何拷贝
//! int len = removeDuplicates(nums);
//!
//! // 在函数里修改输入数组对于调用者是可见的。
//! // 根据你的函数返回的长度, 它会打印出数组中 该长度范围内 的所有元素。
//! for (int i = 0; i < len; i++) {
//!     print(nums[i]);
//! }
//! ```
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [1,1,1,2,2,3]
//! - 输出：5, nums = [1,1,2,2,3]
//! - 解释：函数应返回新长度 length = 5, 并且原数组的前五个元素被修改为 1, 1, 2, 2, 3。不需要考虑数组中超出新长度后面的元素。
//!
//! 示例 2：
//! - 输入：nums = [0,0,1,1,1,1,2,3,3]
//! - 输出：7, nums = [0,0,1,1,2,3,3]
//! - 解释：函数应返回新长度 length = 7, 并且原数组的前七个元素被修改为 0, 0, 1, 1, 2, 3, 3。不需要考虑数组中超出新长度后面的元素。
//!
//! ## 提示
//!
//! - 1 <= nums.length <= 3 * 10^4
//! - -10^4 <= nums[i] <= 10^4
//! - nums 已按升序排列

use crate::Solution;

impl Solution {
    pub fn remove_duplicates_ii(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }

        for i in (2 ..nums.len()).rev() {
            if nums[i - 1] == nums[i] && nums[i - 2] == nums[i - 1] {
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
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let k = Solution::remove_duplicates_ii(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(&nums[..k as usize], &[1, 1, 2, 2, 3]);
    }

    #[test]
    fn test_example_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let k = Solution::remove_duplicates_ii(&mut nums);
        assert_eq!(k, 7);
        assert_eq!(&nums[..k as usize], &[0, 0, 1, 1, 2, 3, 3]);
    }
}
