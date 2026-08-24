use crate::Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;

        fn search(i: usize, j: usize, grid: &mut Vec<Vec<char>>) {
            let mut stack = vec![(i, j)];
            while !stack.is_empty() {
                let k = stack.pop().unwrap();
                if k.0 >= 0 && k.0 < grid.len() && k.1 >= 0 && k.1 < grid[0].len() && grid[k.0][k.1] == '1' {
                    stack.push((k.0 - 1, k.1));
                    stack.push((k.0 + 1, k.1));
                    stack.push((k.0, k.1 - 1));
                    stack.push((k.0, k.1 + 1));
                    grid[k.0][k.1] = '2';
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
