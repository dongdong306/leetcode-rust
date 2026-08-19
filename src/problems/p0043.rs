use crate::Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut result = String::from("0");
        let mut w = 0;
        for c in num2.chars().rev() {
            let mut r: i32 = 0;
            let c: i32 = (c as u8 - b'0') as i32;
            let mut k = 0;
            let mut res = Vec::new();
            for ch in num1.bytes().rev() {
                let ch = (ch - b'0') as i32;
                let r = c * ch + k;
                res.push(((r % 10) as u8 + b'0') as char);
                k = r / 10;
            }

            if k != 0 {
                res.push((b'0' + k as u8) as char);
            }

            let mut t = res.iter().rev().collect::<String>();
            for i in 0 .. w {
                t.push('0')
            }
            w += 1;

            result = add(result.as_bytes(), t.as_bytes());
        }

        return result.to_string();
    }
}

pub fn add(num1: &[u8], num2: &[u8]) -> String {
    let mut i = 0;
    let mut result = Vec::new();
    let mut f = 0;
    while i < num1.len() && i < num2.len() {
        let c1 = num1[num1.len() - i - 1] - b'0';
        let c2 = num2[num2.len() - i - 1] - b'0';
        let c = c1 + c2 + f;
        result.push((c % 10 + b'0') as char);

        if c > 9 {
            f = 1
        } else {
            f = 0
        }
        i += 1;
    }

    while i < num1.len() {
        let c1 = num1[num1.len() - i - 1] - b'0';
        let c = c1 + f;
        result.push((c % 10 + b'0') as char);

        if c > 9 {
            f = 1
        } else {
            f = 0
        }
        i += 1;
    }

    while i < num2.len() {
        let c1 = num2[num2.len() - i - 1] - b'0';
        let c = c1 + f;
        result.push((c % 10 + b'0') as char);

        if c > 9 {
            f = 1
        } else {
            f = 0
        }
        i += 1;
    }

    if f != 0 {
        result.push('1');
    }

    return result.iter().rev().collect::<String>();
}
