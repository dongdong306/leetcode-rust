use crate::Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        if nums.len() == 0 {
            return res;
        } else if nums.len() == 1 {
            res.push(nums[0].to_string());
            return res;
        }

        let mut first = nums[0];
        let mut last = nums[0];
        for i in 1 .. nums.len() {
            let num = nums[i];
            if num != last + 1 {
                if first == last {
                    res.push(first.to_string());
                } else {
                    res.push(format!("{}->{}", first, last));
                }
                first = num;
            }
            last = num;
        }

        if first == last {
            res.push(first.to_string());
        } else {
            res.push(format!("{}->{}", first, last));
        }

        return res
    }
}
