use std::cmp::{max, min};
use crate::Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {

        let mut array = citations;

        array.sort_by(|a, b| a.cmp(b));
        let n = array.len();
        let mut result = 0;
        for (index, k) in array.iter().enumerate() {
            let j = (n - index) as i32;
            if j >= *k {
                result = max(*k, result)
            } else {
                result = max(j, result)
            }
        }

        return result;
    }
}
