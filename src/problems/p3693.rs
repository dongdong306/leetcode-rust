//! LeetCode 第3693题：爬楼梯 II
//!
//! 题目链接：https://leetcode.cn/problems/climbing-stairs-ii/
//!
//! 你正在爬一个有 n + 1 级台阶的楼梯，台阶编号从 0 到 n。
//!
//! 你还得到了一个长度为 n 的 下标从 1 开始 的整数数组 costs，其中 costs[i] 是第 i 级台阶的成本。
//!
//! 从第 i 级台阶，你 只能 跳到第 i + 1、i + 2 或 i + 3 级台阶。从第 i 级台阶跳到第 j 级台阶的成本定义为：costs[j] + (j - i)²
//!
//! 你从第 0 级台阶开始，初始 cost = 0。
//!
//! 返回到达第 n 级台阶所需的 最小 总成本。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：n = 4, costs = [1,2,3,4]
//! - 输出：13
//! - 解释：一个最优路径是 0 → 1 → 2 → 4
//!   - 0 → 1：costs[1] + (1 - 0)² = 1 + 1 = 2
//!   - 1 → 2：costs[2] + (2 - 1)² = 2 + 1 = 3
//!   - 2 → 4：costs[4] + (4 - 2)² = 4 + 4 = 8
//!   - 最小总成本为 2 + 3 + 8 = 13
//!
//! 示例 2：
//! - 输入：n = 4, costs = [5,1,6,2]
//! - 输出：11
//! - 解释：一个最优路径是 0 → 2 → 4
//!   - 0 → 2：costs[2] + (2 - 0)² = 1 + 4 = 5
//!   - 2 → 4：costs[4] + (4 - 2)² = 2 + 4 = 6
//!   - 最小总成本为 5 + 6 = 11
//!
//! 示例 3：
//! - 输入：n = 3, costs = [9,8,3]
//! - 输出：12
//! - 解释：最优路径是 0 → 3，总成本 = costs[3] + (3 - 0)² = 3 + 9 = 12
//!
//! ## 提示
//!
//! - 1 <= n == costs.length <= 10^5
//! - 1 <= costs[i] <= 10^4

use crate::Solution;

impl Solution {
    pub fn climb_stairs_ii(n: i32, costs: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut dp = vec!(i32::MAX; (n + 3) as usize);
        dp[0] = 0;
        for i in 0..n {
            let mut t = 0;
            for j in 0..3 {
                let m = (i + j + 1) as usize;
                if m <= costs.len() {
                    let cost = dp[i as usize] + costs[m - 1] + (m as i32 - i).pow(2);
                    dp[m] = dp[m].min(cost);
                }
            }
        }

        return dp[n as usize];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::climb_stairs_ii(4, vec![1, 2, 3, 4]), 13);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::climb_stairs_ii(4, vec![5, 1, 6, 2]), 11);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::climb_stairs_ii(3, vec![9, 8, 3]), 12);
    }
}
