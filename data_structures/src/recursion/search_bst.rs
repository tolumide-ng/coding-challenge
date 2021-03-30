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
use std::rc::Rc;

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(ref the_root) => {
            if the_root.as_ref().borrow().val == val {
                return root;
            }

            // search_bst(the_root.as_ref().borrow().left.as_ref(), val);
            if the_root.as_ref().borrow().left.is_some() {
                search_bst(
                    Some(Rc::clone(
                        &the_root.as_ref().borrow().left.as_ref().unwrap(),
                    )),
                    val,
                );
            }

            if the_root.as_ref().borrow().right.is_some() {
                search_bst(
                    Some(Rc::clone(
                        &the_root.as_ref().borrow().right.as_ref().unwrap(),
                    )),
                    val,
                );
            }
        }
        None => {
            return None;
        }
    }
    return None;
}

// pub fn recursive_search(root: Option<Rc<RefCell<TreeNode>>>, val: i32) {}
