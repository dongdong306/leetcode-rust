//! LeetCode 第202题：快乐数
//!
//! 题目链接：https://leetcode.cn/problems/happy-number/
//!
//! 编写一个算法来判断一个数 n 是不是快乐数。
//! 「快乐数」定义为：
//! - 对于一个正整数，每一次将该数替换为它每个位置上的数字的平方和。
//! - 然后重复这个过程直到这个数变为 1，也可能是无限循环但始终变不到 1。
//! - 如果这个过程结果为 1，那么这个数就是快乐数。
//! 如果 n 是快乐数就返回 true ；不是，则返回 false。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：n = 19
//! - 输出：true
//! - 解释：1^2 + 9^2 = 82，8^2 + 2^2 = 68，6^2 + 8^2 = 100，1^2 + 0^2 + 0^2 = 1
//!
//! 示例 2：
//! - 输入：n = 2
//! - 输出：false
//!
//! ## 提示
//!
//! - 1 <= n <= 2^31 - 1

use crate::Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn f(mut n: i32) -> i32 {
            let mut res = 0;
            while n != 0 {
                let i = n % 10;
                res += i * i;
                n = n / 10;
            }

            return res;
        }

        let mut slow = f(n);
        let mut fast = f(slow);
        while slow != fast {
            slow = f(slow);
            fast = f(fast);
            fast = f(fast);
        }

        return if slow == 1 { true } else { false };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert!(Solution::is_happy(19));
    }

    #[test]
    fn test_example_2() {
        assert!(!Solution::is_happy(2));
    }
}
