//! LeetCode 第79题：单词搜索
//!
//! 题目链接：https://leetcode.cn/problems/word-search/
//!
//! 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。
//!
//! 单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中"相邻"单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
//! - 输出：true
//!
//! 示例 2：
//! - 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
//! - 输出：true
//!
//! 示例 3：
//! - 输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
//! - 输出：false
//!
//! ## 提示
//!
//! - m == board.length
//! - n == board[i].length
//! - 1 <= m, n <= 6
//! - 1 <= word.length <= 15
//! - board 和 word 仅由大小写英文字母组成
//!
//! ## 进阶
//!
//! 你可以使用搜索剪枝的技术来使解决方案更优，是否可以？

use crate::Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn walk(
            board: &[Vec<char>],
            flag: &mut [Vec<bool>],
            i: isize,
            j: isize,
            word: &[char],
            l: usize,
        ) -> bool {
            if l == word.len() {
                return true;
            }
            if i < 0 || j < 0 {
                return false;
            }
            let (i, j) = (i as usize, j as usize);
            if i >= board.len() || j >= board[0].len() || flag[i][j] || board[i][j] != word[l] {
                return false;
            }

            flag[i][j] = true;
            let found = DIRS
                .iter()
                .any(|&(di, dj)| walk(board, flag, i as isize + di, j as isize + dj, word, l + 1));
            flag[i][j] = false;
            found
        }

        let word: Vec<char> = word.chars().collect();
        let mut flag = vec![vec![false; board[0].len()]; board.len()];

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if walk(&board, &mut flag, i as isize, j as isize, &word, 0) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn board() -> Vec<Vec<char>> {
        vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ]
    }

    #[test]
    fn test_example_1() {
        assert!(Solution::exist(board(), "ABCCED".to_string()));
    }

    #[test]
    fn test_example_2() {
        assert!(Solution::exist(board(), "SEE".to_string()));
    }

    #[test]
    fn test_example_3() {
        assert!(!Solution::exist(board(), "ABCB".to_string()));
    }
}
