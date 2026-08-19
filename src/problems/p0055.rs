use crate::Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max: usize = 0;

        for i in 0..nums.len() {
            if i > max {
                return false;
            }
            let k = i + nums[i] as usize;
            if k > max {
                max = k;
            }
        }

        return true;
    }
}

