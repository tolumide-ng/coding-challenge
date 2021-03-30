#[derive(Default, Debug)]
pub struct MyQueue {
    pub inbox: Vec<i32>,
    pub outbox: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push(&mut self, x: i32) {
        self.inbox.push(x);
    }

    pub fn pop(&mut self, _x: i32) {
        if let Some(a) = self.outbox.pop() {
            self.outbox.push(a);
        } else {
            while let Some(a) = self.inbox.pop() {
                self.outbox.push(a);
            }

            self.outbox.pop().unwrap_or(-1);
        }
    }

    pub fn peek(&self) -> i32 {
        *self.outbox.last().or(self.inbox.first()).unwrap_or(&-1)
    }

    pub fn empty(&self) -> bool {
        self.inbox.is_empty() && self.outbox.is_empty()
    }
}
