// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        let mut sum_stack = vec![];
        let mut node_stack = vec![];
        if root.is_none() {
            return false;
        }
        sum_stack.push(target_sum - root.as_ref().unwrap().borrow().val);
        node_stack.push(Some(Rc::clone(&root.as_ref().unwrap())));
        while node_stack.len() > 0 {
            let mut curr_node = node_stack.pop().unwrap();
            let mut curr_sum = sum_stack.pop().unwrap();
            match curr_node {
                Some(curr_node) => {
                    let root = curr_node.borrow();
                    if root.left.is_none() && root.right.is_none() && curr_sum == 0 {
                        return true;
                    };
                    if root.left.is_some() {
                        node_stack.push(Some(Rc::clone(&root.left.as_ref().unwrap())));
                        sum_stack.push(curr_sum - root.left.as_ref().unwrap().borrow().val);
                    }
                    if root.right.is_some() {
                        node_stack.push(Some(Rc::clone(&root.right.as_ref().unwrap())));
                        sum_stack.push(curr_sum - root.right.as_ref().unwrap().borrow().val);
                    }
                }
                None => {}
            }
        }
        return false;
    }
}
