//! LeetCode 第77题：组合
//!
//! 题目链接：https://leetcode.cn/problems/combinations/
//!
//! 给定两个整数 n 和 k，返回范围 [1, n] 中所有可能的 k 个数的组合。
//!
//! 你可以按 任何顺序 返回答案。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：n = 4, k = 2
//! - 输出：[[2,4],[3,4],[2,3],[1,2],[1,3],[1,4]]
//!
//! 示例 2：
//! - 输入：n = 1, k = 1
//! - 输出：[[1]]
//!
//! ## 提示
//!
//! - 1 <= n <= 20
//! - 1 <= k <= n

use crate::Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut path = Vec::new();
        f(n, k, 1, &mut path, &mut res);
        res
    }
}

fn f(n: i32, k: i32, s: i32, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if path.len() == k as usize {
        res.push(path.clone());
        return;
    }

    for i in s..n + 1 {
        path.push(i);
        f(n, k, i+1, path, res);
        path.pop();

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::combine(4, 2),
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 4],
                vec![2, 3],
                vec![2, 4],
                vec![3, 4],
            ]
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(Solution::combine(1, 1), vec![vec![1]]);
    }
}
