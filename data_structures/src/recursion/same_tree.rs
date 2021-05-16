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

use std::cell::RefCell;
use std::{collections::VecDeque, rc::Rc};
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack_p: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    let mut stack_q: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();

    let mut is_equal = true;

    if p.is_none() && q.is_none() {
        return true;
    }

    if (p.is_none() && q.is_some()) || (p.is_some() && q.is_none()) {
        return false;
    }

    stack_p.push_back(Rc::clone(p.as_ref().unwrap()));
    stack_q.push_back(Rc::clone(q.as_ref().unwrap()));

    while !stack_p.is_empty() && !stack_q.is_empty() && is_equal {
        let pop_p = stack_p.pop_front();
        let pop_q = stack_q.pop_front();

        match pop_p {
            Some(ref the_p) => match pop_q {
                Some(ref the_q) => {
                    if the_p.as_ref().borrow().val != the_q.as_ref().borrow().val {
                        is_equal = false;
                    } else {
                        // the left

                        match the_p.as_ref().borrow().left {
                            Some(ref the_p_left) => match the_q.as_ref().borrow().left {
                                Some(ref the_q_left) => {
                                    stack_p.push_back(Rc::clone(the_p_left));

                                    stack_q.push_back(Rc::clone(the_q_left));
                                }
                                None => {
                                    is_equal = false;
                                }
                            },
                            None => {
                                if the_q.as_ref().borrow().left.is_some() {
                                    is_equal = false
                                }
                            }
                        }

                        match the_p.as_ref().borrow().right {
                            Some(ref the_p_right) => match the_q.as_ref().borrow().right {
                                Some(ref the_q_right) => {
                                    stack_p.push_back(Rc::clone(the_p_right));

                                    stack_q.push_back(Rc::clone(the_q_right))
                                }
                                None => {
                                    is_equal = false;
                                }
                            },
                            None => {
                                if the_q.as_ref().borrow().right.is_some() {
                                    is_equal = false;
                                }
                            }
                        }
                    }
                }
                None => {
                    is_equal = false;
                }
            },
            None => {
                if pop_q.as_ref().is_none() {
                    is_equal = false;
                }
            }
        }
    }

    return is_equal;
}
