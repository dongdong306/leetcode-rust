//! LeetCode 第3963题：构造恰好一条路径的网格
//!
//! 题目链接：https://leetcode.cn/problems/create-grid-with-exactly-one-path/
//!
//! 给你两个整数 m 和 n，分别表示网格的行数和列数。
//!
//! 请你构造任意一个只包含字符 '.' 和 '#' 的 m x n 网格，其中：
//!
//! - '.' 表示空单元格。
//! - '#' 表示障碍物单元格。
//!
//! 有效路径是满足以下条件的空单元格序列：
//!
//! - 从左上角单元格 (0, 0) 开始。
//! - 在右下角单元格 (m - 1, n - 1) 结束。
//! - 只能向：
//!   - 右移动，从 (i, j) 到 (i, j + 1)，或者
//!   - 下移动，从 (i, j) 到 (i + 1, j)。
//!
//! 返回任意一个从左上角到右下角恰好只有一条有效路径的网格。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：m = 2, n = 3
//! - 输出：["..#","#.."]
//! - 解释：唯一的有效路径是：(0,0) → (0,1) → (1,1) → (1,2)
//!
//! 示例 2：
//! - 输入：m = 3, n = 3
//! - 输出：["..#","#..","##."]
//! - 解释：唯一的有效路径是：(0,0) → (0,1) → (1,1) → (1,2) → (2,2)
//!
//! 示例 3：
//! - 输入：m = 1, n = 4
//! - 输出：["...."]
//! - 解释：唯一的有效路径是：(0,0) → (0,1) → (0,2) → (0,3)
//!
//! ## 提示
//!
//! - 1 <= m, n <= 25

use crate::Solution;

impl Solution {
    pub fn create_grid(m: i32, n: i32) -> Vec<String> {
        let mut result = Vec::new();

        result.push(".".repeat(n as usize));
        for i in 1..m {
            result.push("#".repeat((n- 1) as usize) + ".")
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 统计网格中从 (0,0) 到 (m-1,n-1) 只能向右/向下移动的有效路径条数
    fn count_paths(grid: &[String]) -> i64 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![0i64; n]; m];
        for i in 0..m {
            for j in 0..n {
                if grid[i].as_bytes()[j] != b'.' {
                    continue;
                }
                if i == 0 && j == 0 {
                    dp[i][j] = 1;
                }
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if j > 0 {
                    dp[i][j] += dp[i][j - 1];
                }
            }
        }
        dp[m - 1][n - 1]
    }

    /// 验证构造结果是否恰好只有一条有效路径
    fn check(m: i32, n: i32) {
        let grid = Solution::create_grid(m, n);
        assert_eq!(grid.len(), m as usize);
        assert_eq!(grid[0].len(), n as usize);
        assert!(grid[0].starts_with('.'));
        assert!(grid[grid.len() - 1].ends_with('.'));
        assert_eq!(count_paths(&grid), 1, "网格应恰好只有一条有效路径");
    }

    #[test]
    fn test_example_1() {
        check(2, 3);
    }

    #[test]
    fn test_example_2() {
        check(3, 3);
    }

    #[test]
    fn test_example_3() {
        check(1, 4);
    }

    #[test]
    fn test_single_cell() {
        check(1, 1);
    }
}
