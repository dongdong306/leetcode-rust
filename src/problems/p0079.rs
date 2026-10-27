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
