use std::collections::HashSet;

use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();

        for n in nums.iter() {
            if !set.insert(n) {
                return true;
            }
        }

        return false;
    }
}
