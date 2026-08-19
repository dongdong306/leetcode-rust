use std::cmp::Ordering;

use crate::Solution;

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let mut temp_result = Vec::new();
        let mut array = [false; 10];
        fn check(arr: &[bool; 10], k: i32) -> i32 {
            let mut num = 0;
            for a in arr {
                if *a == true {
                    num += 1;
                }
            }

            return num;
        };

        fn print_time(arr: &[bool]) -> i32 {
            let mut sum = 0;
            for (i, a) in arr.iter().enumerate() {
                if *a {
                    sum += 2_i32.pow(i as u32);
                }
            }

            return sum;
        }

        fn rec(arr: &mut [bool; 10], n: usize, k: i32, result: &mut Vec<(i32, i32)>) {
            if check(&arr, k) == k {
                let m = print_time(&arr[0..6]);
                let h = print_time(&arr[6..]);
                if (m < 60 && h < 12) {
                    result.push((h, m));
                }
                return;
            }
            if n == 10 {
                return;
            }

            arr[n] = true;
            rec(arr, n + 1, k, result);
            arr[n] = false;
            rec(arr, n + 1, k, result);
        }

        rec(&mut array, 0, turned_on, &mut temp_result);

        temp_result
            .sort_by(|a, b| {
                if a.0 < b.0 {
                    Ordering::Less
                } else if a.0 > b.0 {
                    Ordering::Greater
                } else {
                    a.1.cmp(&b.1)
                }
            });

        temp_result.iter().map(|it|format!("{}:{:02}", it.0, it.1)).collect()
    }
}
