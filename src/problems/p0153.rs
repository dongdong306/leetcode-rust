use crate::Solution;

// LeetCode 153: 寻找旋转排序数组中的最小值
// 已知一个长度为 n 的升序数组在某一个未知的下标 k（0 <= k < n）处进行了旋转，
// 即在原来的位置 [0, k] 和 [k + 1, n - 1] 两段仍然分别保持升序，
// 但整体不再有序。请找出并返回数组中的最小元素。
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut res = std::i32::MAX;
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut m= 0;

        while l < r {
            m = (l + r) / 2;
            if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = m;
            }
        }

        return nums[m];
    }
}
