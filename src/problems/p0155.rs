
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
