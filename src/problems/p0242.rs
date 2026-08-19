//! LeetCode 第242题：有效的字母异位词
//!
//! 题目链接：https://leetcode.cn/problems/valid-anagram/
//!
//! 给定两个字符串 s 和 t ，编写一个函数来判断 t 是否是 s 的字母异位词。
//! 注意：若 s 和 t 中每个字符出现的次数都相同，则称 s 和 t 互为字母异位词。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：s = "anagram", t = "nagaram"
//! - 输出：true
//!
//! 示例 2：
//! - 输入：s = "rat", t = "car"
//! - 输出：false
//!
//! ## 提示
//!
//! - 1 <= s.length, t.length <= 5 * 10^4
//! - s 和 t 仅包含小写字母

use crate::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut a = [0; 256];

        for b in s.as_bytes() {
            a[*b as usize] += 1;
        }

        for b in t.as_bytes() {
            a[*b as usize] -= 1;
        }

        for i in a {
            if i != 0 {
                return false;
            }
        }

        true
    }
}
