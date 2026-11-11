//! LeetCode 第228题：汇总区间
//!
//! 题目链接：https://leetcode.cn/problems/summary-ranges/
//!
//! 给定一个 无重复元素 的 有序 整数数组 nums 。
//!
//! 返回 恰好覆盖数组中所有数字 的 最小有序 区间范围列表 。也就是说，nums 的每个元素都恰好被某个范围所覆盖，并且不存在属于某个范围但不属于 nums 的数字 x 。
//!
//! 列表中的每个区间范围 [a,b] 应该按以下格式输出：
//!
//! - "a->b" ，如果 a != b
//! - "a" ，如果 a == b
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：nums = [0,1,2,4,5,7]
//! - 输出：["0->2","4->5","7"]
//! - 解释：区间范围是 [0,2]、[4,5] 和 [7]，可以认为 "0" 和 "7" 是自覆盖的区间。
//!
//! 示例 2：
//! - 输入：nums = [0,2,3,4,6,8,9]
//! - 输出：["0","2->4","6","8->9"]
//! - 解释：区间范围是 [0]、[2,4]、[6] 和 [8,9]。
//!
//! ## 提示
//!
//! - 0 <= nums.length <= 20
//! - -2^31 <= nums[i] <= 2^31 - 1
//! - nums 中的所有值 互不相同
//! - nums 按升序排列

use crate::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        if nums.len() == 0 {
            return res;
        } else if nums.len() == 1 {
            res.push(nums[0].to_string());
            return res;
        }

        let mut first = nums[0];
        let mut last = nums[0];
        for i in 1 .. nums.len() {
            let num = nums[i];
            if num != last + 1 {
                if first == last {
                    res.push(first.to_string());
                } else {
                    res.push(format!("{}->{}", first, last));
                }
                first = num;
            }
            last = num;
        }

        if first == last {
            res.push(first.to_string());
        } else {
            res.push(format!("{}->{}", first, last));
        }

        return res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec!["0->2", "4->5", "7"]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec!["0", "2->4", "6", "8->9"]
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(Solution::summary_ranges(vec![]), Vec::<String>::new());
    }
}
