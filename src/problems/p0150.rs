//! LeetCode 第150题：逆波兰表达式求值
//!
//! 题目链接：https://leetcode.cn/problems/evaluate-reverse-polish-notation/
//!
//! 给你一个字符串数组 tokens，表示一个根据逆波兰表示法表示的算术表达式。
//!
//! 请你计算该表达式。返回一个表示表达式值的整数。
//!
//! 注意：
//!
//! - 有效的算符为 '+'、'-'、'*' 和 '/'。
//! - 每个操作数（运算对象）都可以是一个整数或者另一个表达式。
//! - 两个整数之间的除法总是 向零截断。
//! - 表达式中不含除零运算。
//! - 输入是一个根据逆波兰表示法表示的算术表达式。
//! - 答案及所有中间计算结果可以用 32 位整数表示。
//!
//! ## 示例
//!
//! 示例 1：
//! - 输入：tokens = ["2","1","+","3","*"]
//! - 输出：9
//! - 解释：该算式转化为常见的中缀算术表达式为：((2 + 1) * 3) = 9
//!
//! 示例 2：
//! - 输入：tokens = ["4","13","5","/","+"]
//! - 输出：6
//! - 解释：该算式转化为常见的中缀算术表达式为：(4 + (13 / 5)) = 6
//!
//! 示例 3：
//! - 输入：tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
//! - 输出：22
//! - 解释：该算式转化为常见的中缀算术表达式为：
//!   ((10 * (6 / ((9 + 3) * -11))) + 17) + 5 = 22
//!
//! ## 提示
//!
//! - 1 <= tokens.length <= 10^4
//! - tokens[i] 是一个算符（"+"、"-"、"*" 或 "/"），或是在范围 [-200, 200] 内的一个整数

use crate::Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack:Vec<i32> = Vec::new();

        fn pop_operate(stack: &mut Vec<i32>, operator: fn(i32, i32) -> i32) {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();
            stack.push(operator(a, b))
        };

        for token in tokens {
            match token.as_str() {
                "+" => pop_operate(&mut stack, |a, b| a + b),
                "-" => pop_operate(&mut stack, |a, b| a - b),
                "*" => pop_operate(&mut stack, |a, b| a * b),
                "/" => pop_operate(&mut stack, |a, b| a / b),
                _ => {
                    stack.push(token.parse::<i32>().unwrap());
                }
            }
        }

        return stack.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "3".to_string(),
                "*".to_string()
            ]),
            9
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "4".to_string(),
                "13".to_string(),
                "5".to_string(),
                "/".to_string(),
                "+".to_string()
            ]),
            6
        );
    }

    #[test]
    fn test_example_3() {
        assert_eq!(
            Solution::eval_rpn(vec![
                "10".to_string(),
                "6".to_string(),
                "9".to_string(),
                "3".to_string(),
                "+".to_string(),
                "-11".to_string(),
                "*".to_string(),
                "/".to_string(),
                "*".to_string(),
                "17".to_string(),
                "+".to_string(),
                "5".to_string(),
                "+".to_string()
            ]),
            22
        );
    }
}
