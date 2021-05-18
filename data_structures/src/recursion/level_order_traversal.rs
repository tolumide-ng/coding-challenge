// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::rc::Rc;
use std::{cell::RefCell, collections::VecDeque};

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = vec![];
    match root {
        Some(ref the_root) => {
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

            queue.push_back(Rc::clone(the_root));

            while !queue.is_empty() {
                let mut queue_len = queue.len();

                let mut current_level: Vec<i32> = vec![];
                while queue_len > 0 {
                    queue_len -= 1;

                    let current = queue.pop_front();
                    match current {
                        Some(ref the_current) => {
                            if the_current.as_ref().borrow().left.is_some() {
                                queue.push_back(Rc::clone(
                                    the_current.as_ref().borrow().left.as_ref().unwrap(),
                                ));
                            }

                            if the_current.as_ref().borrow().right.is_some() {
                                queue.push_back(Rc::clone(
                                    the_current.as_ref().borrow().right.as_ref().unwrap(),
                                ));
                            }

                            current_level.push(the_current.as_ref().borrow().val);
                        }
                        None => {}
                    }
                }

                output.push(current_level);
            }
        }
        None => {}
    }

    return output;
}
