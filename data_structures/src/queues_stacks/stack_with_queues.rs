// #![allow(warnings, unused)]
#![allow(dead_code, unused_imports)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// https://www.youtube.com/watch?v=BS3dvTI6yNg&t=331s

#[derive(Debug, Default)]
struct MyStack {
    queue_one: Rc<RefCell<VecDeque<i32>>>,
    queue_two: Rc<RefCell<VecDeque<i32>>>,
}

impl MyStack {
    /** Initialize your data structure here. */
    // #[allow(dead_code)]
    pub fn new() -> Self {
        Default::default()
    }

    /** Push element x onto stack. */
    // #[allow(dead_code)]
    pub fn push(&mut self, x: i32) {
        self.queue_one.as_ref().borrow_mut().push_back(x);

        while !self.queue_two.as_ref().borrow().is_empty() {
            let current = self.queue_two.as_ref().borrow_mut().pop_front();
            self.queue_one
                .as_ref()
                .borrow_mut()
                .push_back((current.unwrap()));
        }

        self.queue_two = Rc::clone(&self.queue_one);
        self.queue_one = Rc::new(RefCell::new(VecDeque::new()));
    }

    /** Removes the element on top of the stack and returns that element. */
    // #[allow(dead_code)]
    pub fn pop(&self) -> i32 {
        let mut value = -1;
        if self.queue_two.as_ref().borrow().len() > 0 {
            value = self.queue_two.as_ref().borrow_mut().pop_front().unwrap();
        }

        return value;
    }

    /** Get the top element. */
    // #[allow(dead_code)]
    pub fn top(&self) -> i32 {
        let mut value = -1;

        if self.queue_two.as_ref().borrow().front().is_some() {
            value = *self.queue_two.as_ref().borrow().front().unwrap();
        }

        return value;
    }

    /** Returns whether the stack is empty. */
    // #[allow(dead_code)]
    pub fn empty(&self) -> bool {
        self.queue_two.as_ref().borrow().is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_stack_with_queue() {
        let mut v = MyStack::new();
        v.push(4);
        v.push(5);
        v.push(10);

        assert_eq!(v.top(), 10);
        assert_eq!(v.pop(), 10);
        assert!(!v.empty());
git 