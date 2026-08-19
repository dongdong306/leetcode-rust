use crate::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut array: [(usize, i32); 26] = [(0, 0); 26];

        for (i, c) in s.bytes().enumerate() {
            let a = &mut array[(c - b'a') as usize];
            if a.1 == 0 {
                a.0 = i;
            }
            a.1 += 1;
        }

        let mut k = usize::MAX;
        for entry in array {
            if k > entry.0 && entry.1 == 1 {
                k = entry.0;
            }
        }

        if k == usize::MAX { -1 } else { k as i32 }
    }
}
