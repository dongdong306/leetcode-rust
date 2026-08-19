//! LeetCode 第14题：最长公共前缀
//!
//! 题目链接：https://leetcode.cn/problems/longest-common-prefix/
//!
//! 编写一个函数来查找字符串数组中的最长公共前缀。
//! 如果不存在公共前缀，返回空字符串 ""。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：strs = ["flower","flow","flight"]
//! - 输出："fl"
//!
//! 示例 2：
//! - 输入：strs = ["dog","racecar","car"]
//! - 输出：""
//! - 解释：输入不存在公共前缀。
//!
//! ## 提示
//!
//! - 1 <= strs.length <= 200
//! - 0 <= strs[i].length <= 200
//! - strs[i] 仅由小写英文字母组成

use crate::Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut res = String::new();
        let mut strs = strs.iter().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
        strs.sort_by(|a, b| a.len().cmp(&b.len()));

        fn is_same(strs: &Vec<&[u8]>, c: u8, index: usize) -> bool {
            for i in 1..strs.len() {
                if strs[i][index] != c {
                    return false;
                }
            }

            return true
        }

        for i in 0..strs[0].len() {
            let c = strs[0][i];
            if is_same(&strs, c, i) {
                res.push(char::from(c))
            } else {
                break;
            }
        }
        return res
    }
}
