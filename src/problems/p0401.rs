//! LeetCode 第401题：二进制手表
//!
//! 题目链接：https://leetcode.cn/problems/binary-watch/
//!
//! 二进制手表顶部有 4 个 LED 代表 小时（0-11），底部的 6 个 LED 代表 分钟（0-59）。
//!
//! 每个 LED 代表一个 0 或 1，最低位在右侧。
//!
//! - 例如，下面的二进制手表读取 "4:51" 。
//!
//! 给你一个整数 turnedOn ，表示当前亮着的 LED 的数量，返回二进制手表可以表示的所有可能时间。你可以 按任意顺序 返回答案。
//!
//! 小时不会以零开头：
//!
//! - 例如，"01:00" 是无效的时间，正确的写法应该是 "1:00" 。
//!
//! 分钟 必须由两位数组成，可能会以零开头：
//!
//! - 例如，"10:2" 是无效的时间，正确的写法应该是 "10:02" 。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：turnedOn = 1
//! - 输出：["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
//!
//! 示例 2：
//! - 输入：turnedOn = 9
//! - 输出：[]
//! - 解释：二进制手表只点亮 9 个 LED 是无法表示任何时间的。
//!
//! ## 提示
//!
//! - 0 <= turnedOn <= 10

use std::cmp::Ordering;

use crate::Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut temp_result = Vec::new();
        let mut array = [false; 10];
        fn check(arr: &[bool; 10], k: i32) -> i32 {
            let mut num = 0;
            for a in arr {
                if *a == true {
                    num += 1;
                }
            }

            return num;
        };

        fn print_time(arr: &[bool]) -> i32 {
            let mut sum = 0;
            for (i, a) in arr.iter().enumerate() {
                if *a {
                    sum += 2_i32.pow(i as u32);
                }
            }

            return sum;
        }

        fn rec(arr: &mut [bool; 10], n: usize, k: i32, result: &mut Vec<(i32, i32)>) {
            if check(&arr, k) == k {
                let m = print_time(&arr[0..6]);
                let h = print_time(&arr[6..]);
                if (m < 60 && h < 12) {
                    result.push((h, m));
                }
                return;
            }
            if n == 10 {
                return;
            }

            arr[n] = true;
            rec(arr, n + 1, k, result);
            arr[n] = false;
            rec(arr, n + 1, k, result);
        }

        rec(&mut array, 0, turned_on, &mut temp_result);

        temp_result
            .sort_by(|a, b| {
                if a.0 < b.0 {
                    Ordering::Less
                } else if a.0 > b.0 {
                    Ordering::Greater
                } else {
                    a.1.cmp(&b.1)
                }
            });

        temp_result.iter().map(|it|format!("{}:{:02}", it.0, it.1)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut result = Solution::read_binary_watch(1);
        result.sort();
        assert_eq!(
            result,
            vec![
                "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00",
            ]
        );
    }

    #[test]
    fn test_example_2() {
        assert!(Solution::read_binary_watch(9).is_empty());
    }

    #[test]
    fn test_zero_led() {
        assert_eq!(Solution::read_binary_watch(0), vec!["0:00"]);
    }
}
