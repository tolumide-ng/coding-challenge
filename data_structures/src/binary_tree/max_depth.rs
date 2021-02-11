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
use std::rc::Rc;

struct Solution {};


impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let mut left: Option<Rc<RefCell<TreeNode>>> = None;
        if root.as_ref().unwrap().borrow().left.is_some() {
            left = Some(Rc::clone(
                &root.as_ref().unwrap().borrow().left.as_ref().unwrap(),
            ))
        }
        let mut left_result = Solution::max_depth(left);

        let mut right: Option<Rc<RefCell<TreeNode>>> = None;
        if root.as_ref().unwrap().borrow().right.is_some() {
            right = Some(Rc::clone(
                &root.as_ref().unwrap().borrow().right.as_ref().unwrap(),
            ))
        }
        let mut right_result = Solution::max_depth(right);

        return std::cmp::max(right_result, left_result) + 1;
    }
}

// solution from leetcode
// use std::rc::Rc;
// use std::cell::RefCell;
use std::cmp::max;
impl Solution {
    pub fn max_depth_second(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::recursive(root)
    }

    fn recursive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }

        let root = root.unwrap().replace(TreeNode::new(0));
        return 1 + max(
            Solution::recursive(root.left),
            Solution::recursive(root.right),
        );
    }
}
