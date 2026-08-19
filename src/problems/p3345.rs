//! LeetCode 第3345题：最小可整除数位乘积 I
//!
//! 题目链接：https://leetcode.cn/problems/smallest-divisible-digit-product-i/
//!
//! 给你两个整数 n 和 t。请你返回大于等于 n 的 最小 整数，且该整数的 各数位之积 能被 t 整除。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：n = 10, t = 2
//! - 输出：10
//! - 解释：10 的数位乘积为 0，可以被 2 整除，所以它是大于等于 10 且满足题目要求的最小整数。
//!
//! 示例 2：
//! - 输入：n = 15, t = 3
//! - 输出：16
//! - 解释：16 的数位乘积为 6，可以被 3 整除，所以它是大于等于 15 且满足题目要求的最小整数。
//!
//! ## 提示
//!
//! - 1 <= n <= 100
//! - 1 <= t <= 10

use crate::Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        fn f(mut n: i32) -> i32 {
            if n == 0 {
                return 0;
            }
            let mut res = 1;
            while (n != 0) {
                let t = n % 10;
                res = res * t;
                n = n / 10
            }
            return res;
        }

        let mut n = n;
        loop {
            if f(n) % t == 0 {
                return n;
            }
            n = n + 1;
        }
    }
}
