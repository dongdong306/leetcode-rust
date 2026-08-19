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