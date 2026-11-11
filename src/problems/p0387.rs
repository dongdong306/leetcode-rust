//! LeetCode 第387题：字符串中的第一个唯一字符
//!
//! 题目链接：https://leetcode.cn/problems/first-unique-character-in-a-string/
//!
//! 给定一个字符串 s ，找到 它的第一个不重复的字符，并返回它的索引 。如果不存在，则返回 -1 。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入: s = "leetcode"
//! - 输出: 0
//!
//! 示例 2：
//! - 输入: s = "loveleetcode"
//! - 输出: 2
//!
//! 示例 3：
//! - 输入: s = "aabb"
//! - 输出: -1
//!
//! ## 提示
//!
//! - 1 <= s.length <= 10^5
//! - s 只包含小写字母

use crate::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut array: [(usize, i32); 26] = [(0, 0); 26];

        for (i, c) in s.bytes().enumerate() {
            let a = &mut array[(c - b'a') as usize];
            if a.1 == 0 {
                a.0 = i;
            }
            a.1 += 1;
        }

        let mut k = usize::MAX;
        for entry in array {
            if k > entry.0 && entry.1 == 1 {
                k = entry.0;
            }
        }

        if k == usize::MAX { -1 } else { k as i32 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::first_uniq_char("aabb".to_string()), -1);
    }
}
