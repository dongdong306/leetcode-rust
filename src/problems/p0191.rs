use crate::Solution;

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        loop {
            if n == 0 {
                return res;
            }
            if n % 2 == 1 {
                res += 1;
            }
            n = n / 2;
        }
    }
}
