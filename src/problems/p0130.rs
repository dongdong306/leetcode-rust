//! LeetCode 第130题：被围绕的区域
//!
//! 题目链接：https://leetcode.cn/problems/surrounded-regions/
//!
//! 给你一个 m x n 的矩阵 board，由若干字符 'X' 和 'O' 组成，找到所有被 'X' 围绕的区域，
//! 并将这些区域里所有的 'O' 用 'X' 填充。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：board = [["X","X","X","X"],["X","O","O","X"],["X","X","O","X"],["X","O","X","X"]]
//! - 输出：[["X","X","X","X"],["X","X","X","X"],["X","X","X","X"],["X","O","X","X"]]
//! - 解释：被围绕的区间不会存在于边界上，换句话说，任何边界上的 'O' 都不会被填充为 'X'。
//!   任何不在边界上，或不与边界上的 'O' 相连的 'O' 最终都会被填充为 'X'。
//!   如果两个元素在水平或垂直方向相邻，则称它们是“相连”的。
//!
//! 示例 2：
//! - 输入：board = [["X"]]
//! - 输出：[["X"]]
//!
//! ## 提示
//!
//! - m == board.length
//! - n == board[i].length
//! - 1 <= m, n <= 200
//! - board[i][j] 为 'X' 或 'O'

use crate::Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        fn f(i: usize, j: usize, board: &mut Vec<Vec<char>>) {
            let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let mut flag = false;
            let mut stack: Vec<(usize, usize)> = Vec::new();
            let mut k = 0;
            if board[i][j] == 'O' {
                stack.push((i, j));
            }

            while k < stack.len()  {
                let (x, y) = stack[k];
                if board[x][y] == 'O' {
                    board[x][y] = 'o';
                    for direct in directions.iter() {
                        let di = x as i32 + direct.0;
                        let dj = y as i32 + direct.1;
                        if di >= board.len() as i32 || dj >= board[0].len() as i32 || di < 0 || dj < 0 {
                            flag = true;
                        } else if board[di as usize][dj as usize] == 'O' {
                            stack.push((di as usize, dj as usize));
                        }
                    }
                }

                k += 1;
            }

            for s in stack.iter() {
                if !flag {
                    board[s.0][s.1] = 'X';
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                f(i, j, board);
                if board[i][j] == 'o' {
                    board[i][j] = 'O';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ]
        );
    }

    #[test]
    fn test_example_2() {
        let mut board = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(board, vec![vec!['X']]);
    }
}
