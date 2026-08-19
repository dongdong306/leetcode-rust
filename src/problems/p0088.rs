use crate::Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut j = m as usize;
        let mut k = n as usize;
        for i in (0 .. nums1.len()).rev() {
            if j == 0 || k == 0 {
                break;
            }

            let a = nums1[j - 1];
            let b = nums2[k - 1];

            if a > b {
                nums1[i] = a;
                j = j - 1;
            } else {
                nums1[i] = b;
                k = k -1;
            }

        }

        if m == 0 {
            for i in 0 .. nums2.len() {
                nums1[i] = nums2[i];
            }
        }
    }
}
