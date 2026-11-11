
//! LeetCode 第155题：最小栈
//!
//! 题目链接：https://leetcode.cn/problems/min-stack/
//!
//! 设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
//!
//! 实现 MinStack 类：
//!
//! - MinStack() 初始化堆栈对象。
//! - void push(int val) 将元素val推入堆栈。
//! - void pop() 删除堆栈顶部的元素。
//! - int top() 获取堆栈顶部的元素。
//! - int getMin() 获取堆栈中的最小元素。
//!
//! ## 示例
//!
//! 输入：
//! - ["MinStack","push","push","push","getMin","pop","top","getMin"]
//! - [[],[-2],[0],[-3],[],[],[],[]]
//!
//! 输出：[null,null,null,null,-3,null,0,-2]
//!
//! 解释：
//! - MinStack minStack = new MinStack();
//! - minStack.push(-2);
//! - minStack.push(0);
//! - minStack.push(-3);
//! - minStack.getMin();   --> 返回 -3.
//! - minStack.pop();
//! - minStack.top();      --> 返回 0.
//! - minStack.getMin();   --> 返回 -2.
//!
//! ## 提示
//!
//! - -2^31 <= val <= 2^31 - 1
//! - pop、top 和 getMin 操作总是在 非空栈 上调用
//! - 最多调用 3 * 10^4 次 push、pop、top 和 getMin 操作

struct MinStack {
    m_stack: Vec<i32>,
    stack: Vec<i32>
}

impl MinStack {
    fn new() -> Self {
        return MinStack { m_stack: Vec::new(), stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val);
        match self.m_stack.last() {
            Some(x) => {
                if *x > val {
                    self.m_stack.push(val);
                } else {
                    self.m_stack.push(*x);
                }
            }
            None => self.m_stack.push(val)
        }
    }

    fn pop(&mut self) {
        self.m_stack.pop();
        self.stack.pop();
    }

    fn top(&self) -> i32 {
        self.stack[self.stack.len() - 1]
    }

    fn get_min(&self) -> i32 {
        self.m_stack[self.m_stack.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut stack = MinStack::new();
        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        assert_eq!(stack.get_min(), -3);
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}
