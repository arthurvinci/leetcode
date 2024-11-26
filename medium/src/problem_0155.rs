use std::cmp::min;

struct StackNode {
    value: i32,
    minimum_at_current: i32,
}

struct MinStack {
    internal_stack: Vec<StackNode>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            internal_stack: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        let minimum_at_current = self
            .internal_stack
            .last()
            .unwrap_or(&StackNode {
                value: 0,
                minimum_at_current: val,
            })
            .minimum_at_current;
        self.internal_stack.push(StackNode {
            value: val,
            minimum_at_current: min(minimum_at_current, val),
        })
    }

    fn pop(&mut self) {
        self.internal_stack.pop();
    }

    fn top(&self) -> i32 {
        self.internal_stack.last().unwrap().value
    }

    fn get_min(&self) -> i32 {
        self.internal_stack.last().unwrap().minimum_at_current
    }
}
