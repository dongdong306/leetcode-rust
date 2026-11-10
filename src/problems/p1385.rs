//! LeetCode 第1385题：两个数组间的距离值
//!
//! 题目链接：https://leetcode.cn/problems/find-the-distance-value-between-two-arrays/
//!
//! 给你两个整数数组 arr1 ，arr2 和一个整数 d ，请你返回两个数组之间的 距离值 。
//!
//! 「距离值」 定义为符合此距离要求的元素数目：对于元素 arr1[i] ，不存在任何元素 arr2[j] 满足 |arr1[i]-arr2[j]| <= d 。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：arr1 = [4,5,8], arr2 = [10,9,1,8], d = 2
//! - 输出：2
//! - 解释：对于 arr1[0]=4 有 |4-10|=6 > d=2、|4-9|=5 > d=2、|4-1|=3 > d=2、|4-8|=4 > d=2，符合距离要求；
//!   对于 arr1[1]=5 也有 |5-10|=5 > d=2、|5-9|=4 > d=2、|5-1|=4 > d=2、|5-8|=3 > d=2，符合距离要求；
//!   对于 arr1[2]=8 存在 |8-10|=2 <= d=2、|8-8|=0 <= d=2，不符合距离要求。故距离值为 2。
//!
//! 示例 2：
//! - 输入：arr1 = [1,4,2,3], arr2 = [-4,-3,6,10,20,30], d = 3
//! - 输出：2
//!
//! 示例 3：
//! - 输入：arr1 = [2,1,100,3], arr2 = [-5,-2,10,-3,7], d = 6
//! - 输出：1
//!
//! ## 提示
//!
//! - 1 <= arr1.length, arr2.length <= 500
//! - -10^3 <= arr1[i], arr2[j] <= 10^3
//! - 0 <= d <= 100

use crate::Solution;

impl Solution {
    pub fn find_the_distance_value(arr1: Vec<i32>, mut arr2: Vec<i32>, d: i32) -> i32 {
        arr2.sort();

        fn bsearch(arr: &Vec<i32>, k: i32) -> usize {
            let mut l = 0;
            let mut r = arr.len() - 1;
            let mut m;
            if arr[r] < k {
                return r + 1;
            }

            while l < r {
                m = l + (r - l) / 2;
                if k > arr[m] {
                    l = m + 1;
                } else {
                    r = m;
                }
            }

            return l;
        }

        let mut res = 0;
        for i in arr1 {
            let k = bsearch(&arr2, i);
            let mut r = true;

            if k < arr2.len() {
                r &= (arr2[k] - i > d);
            }

            if  k - 1 < arr2.len() {
                r &= (i - arr2[k - 1]> d);
            }

            res += if r { 1 } else { 0 };
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::find_the_distance_value(vec![4, 5, 8], vec![10, 9, 1, 8], 2),
            2
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::find_the_distance_value(
                vec![1, 4, 2, 3],
                vec![-4, -3, 6, 10, 20, 30],
                3
            ),
            2
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::find_the_distance_value(
                vec![2, 1, 100, 3],
                vec![-5, -2, 10, -3, 7],
                6
            ),
            1
        );
    }
}
