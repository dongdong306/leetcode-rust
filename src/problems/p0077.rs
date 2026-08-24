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
