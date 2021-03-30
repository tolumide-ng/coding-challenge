pub struct MinStack {
    min: Option<i32>,
    store: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            min: None,
            store: vec![],
        }
    }

    pub fn push(&mut self, x: i32) {
        match self.min {
            Some(curr_min) => {
                let new_min = std::cmp::min(curr_min, x);
                self.min = Some(new_min)
            }
            None => self.min = Some(x),
        }

        self.store.push(x);
    }

    pub fn pop(&mut self) {
        match self.min {
            Some(curr_min) => {
                let removed = self.store.pop().unwrap();
                if self.store.len() == 0 {
                    self.min = None;
                } else {
                    if removed == curr_min {
                        let new_min = self.store.iter().min().unwrap();

                        self.min = Some(*new_min);
                    }
                }
            }
            None => {}
        }
    }

    pub fn top(&self) -> i32 {
        // top would always be called on non-empty stacks
        return self.store[self.store.len() - 1];
    }

    pub fn get_min(&self) -> i32 {
        // top would always be called on non-empty stacks

        return self.min.unwrap();
    }
}

#[cfg(test)]
mod test_min_stack_cont {
    use super::*;

    #[test]
    fn test_min_stack() {
        let mut stack = MinStack::new();

        stack.push(-2);
        stack.push(0);
        stack.push(-3);
        stack.get_min();
        stack.pop();
        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}
