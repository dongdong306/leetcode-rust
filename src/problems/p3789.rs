//! LeetCode 第3789题：采购的最小花费
//!
//! 题目链接：https://leetcode.cn/problems/minimum-cost-to-acquire-required-items/
//!
//! 给你五个整数 cost1、cost2、costBoth、need1 和 need2。
//!
//! 有三种类型的物品可以购买：
//!
//! - 类型 1 的物品花费 cost1，并仅满足类型 1 的需求 1 个单位。
//! - 类型 2 的物品花费 cost2，并仅满足类型 2 的需求 1 个单位。
//! - 类型 3 的物品花费 costBoth，同时满足类型 1 和类型 2 的需求各 1 个单位。
//!
//! 你需要购买足够的物品，使得：
//!
//! - 满足类型 1 的总需求 至少 为 need1。
//! - 满足类型 2 的总需求 至少 为 need2。
//!
//! 返回满足这些需求的 最小 可能总花费。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：cost1 = 3, cost2 = 2, costBoth = 1, need1 = 3, need2 = 2
//! - 输出：3
//! - 解释：购买 3 个类型 3 的物品，总花费为 3 * 1 = 3，此时类型 1 的总需求满足 3（>= need1 = 3），类型 2 的总需求满足 3（>= need2 = 2）。任何其他有效的购买方案都会花费更多，因此最小总花费为 3。
//!
//! 示例 2：
//! - 输入：cost1 = 5, cost2 = 4, costBoth = 15, need1 = 2, need2 = 3
//! - 输出：22
//! - 解释：购买 need1 = 2 个类型 1 的物品和 need2 = 3 个类型 2 的物品，总花费为 2 * 5 + 3 * 4 = 10 + 12 = 22。任何其他有效的购买方案都会花费更多，因此最小总花费为 22。
//!
//! 示例 3：
//! - 输入：cost1 = 5, cost2 = 4, costBoth = 15, need1 = 0, need2 = 0
//! - 输出：0
//! - 解释：由于不需要任何物品（need1 = need2 = 0），因此无需购买，总花费为 0。
//!
//! ## 提示
//!
//! - 1 <= cost1, cost2, costBoth <= 10^6
//! - 0 <= need1, need2 <= 10^9

use crate::Solution;

impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let cost1 = cost1 as i64;
        let cost2 = cost2 as i64;
        let cost_both = cost_both as i64;
        let need1 = need1 as i64;
        let need2 = need2 as i64;

        let min_need = need1.min(need2);
        let r1: i64 = min_need * cost_both.min(cost1 + cost2);
        let r2: i64 = (need1 - min_need) * cost1.min(cost_both) + (need2 - min_need) * cost2.min(cost_both);

        return r1 + r2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(Solution::minimum_cost(3, 2, 1, 3, 2), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::minimum_cost(5, 4, 15, 2, 3), 22);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(Solution::minimum_cost(5, 4, 15, 0, 0), 0);
    }
}
