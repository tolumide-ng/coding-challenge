use crate::recursion::create_bst::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let found_root: Option<Rc<RefCell<TreeNode>>> = None;

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

    return found_root;
}

// pub fn recursive_search(root: Option<Rc<RefCell<TreeNode>>>, val: i32) {}

#[cfg(test)]
mod test_search_bst_cont {
    #[test]
    fn test_search_bst() {
        // use super::*;

        // let input = vec![1, 2, 3, 4, 5, 6, 7];
    }
}
