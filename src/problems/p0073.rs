//! LeetCode 第73题：矩阵置零
//!
//! 题目链接：https://leetcode.cn/problems/set-matrix-zeroes/
//!
//! 给定一个 m x n 的矩阵，如果一个元素为 0 ，则将其所在行和列全部元素置为 0 。请使用 原地 算法。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：matrix = [[1,1,1],[1,0,1],[1,1,1]]
//! - 输出：[[1,0,1],[0,0,0],[1,0,1]]
//!
//! 示例 2：
//! - 输入：matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
//! - 输出：[[0,0,0,0],[0,4,5,0],[1,3,1,0]]
//!
//! ## 提示
//!
//! - m == matrix.length
//! - n == matrix[0].length
//! - 1 <= m, n <= 200
//! - -2^31 <= matrix[i][j] <= 2^31 - 1
//!
//! ## 进阶
//!
//! - 一个直接的解决方案是使用 O(mn) 的额外空间，但这并不是一个好的解决方案。
//! - 一个简单的改进方案是使用 O(m + n) 的额外空间，但这仍然不是最好的解决方案。
//! - 你能想出一个仅使用常量空间的解决方案吗？

use crate::Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let r_l = matrix.len();
        let c_l = matrix[0].len();
        let mut r = false;
        let mut c = false;

        for i in 0..c_l {
            if matrix[0][i] == 0 {
                r = true
            }
        }

        for i in 0..r_l {
            if matrix[i][0] == 0 {
                c = true
            }
        }

        for i in 0..r_l {
            for j in 0..c_l {
                if matrix[i][j] == 0 {
                    matrix[0][j] = 0;
                    matrix[i][0] = 0;
                }
            }
        }

        for i in 1..c_l {
            if matrix[0][i] == 0 {
                for j in 0..r_l {
                    matrix[j][i] = 0;
                }
            }
        }

        for i in 1..r_l {
            if matrix[i][0] == 0 {
                for j in 0..c_l {
                    matrix[i][j] = 0;
                }
            }
        }

        if r {
            for j in 0..c_l {
                matrix[0][j] = 0;
            }
        }

        if c {
            for j in 0..r_l {
                matrix[j][0] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]]);
    }

    #[test]
    fn test_example_2() {
        let mut matrix = vec![
            vec![0, 1, 2, 0],
            vec![3, 4, 5, 2],
            vec![1, 3, 1, 5],
        ];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(
            matrix,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 4, 5, 0],
                vec![0, 3, 1, 0],
            ]
        );
    }
}