//! LeetCode 第200题：岛屿数量
//!
//! 题目链接：https://leetcode.cn/problems/number-of-islands/
//!
//! 给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。
//!
//! 岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。
//!
//! 此外，你可以假设该网格的四条边均被水包围。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：grid = [["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]
//! - 输出：1
//!
//! 示例 2：
//! - 输入：grid = [["1","1","0","0","0"],["1","1","0","0","0"],["0","0","1","0","0"],["0","0","0","1","1"]]
//! - 输出：3
//! - 解释：示例 2 包含三座岛屿（每座岛屿由水平或竖直方向上相邻的 '1' 连接形成）。
//!
//! ## 提示
//!
//! - m == grid.length
//! - n == grid[i].length
//! - 1 <= m, n <= 300
//! - grid[i][j] 的值为 '0' 或 '1'

use crate::Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;

        fn search(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
            let mut stack = vec![(i, j)];
            while !stack.is_empty() {
                let k = stack.pop().unwrap();
                if grid[k.0][k.1] == '1' {
                    grid[k.0][k.1] = '2';
                    if k.0 > 0 {
                        stack.push((k.0 - 1, k.1));
                    }
                    if k.0 + 1 < grid.len() {
                        stack.push((k.0 + 1, k.1));
                    }
                    if k.1 > 0 {
                        stack.push((k.0, k.1 - 1));
                    }
                    if k.1 + 1 < grid[0].len() {
                        stack.push((k.0, k.1 + 1));
                    }
                }
            }
        }

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == '1' {
                    search(i, j, &mut grid);
                    res += 1;
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(Solution::num_islands(grid), 1);
    }

    #[test]
    fn test_example_2() {
        let grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }
}
