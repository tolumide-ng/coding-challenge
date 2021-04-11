// Definition for a binary tree node.
use crate::recursion::create_bst::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // return check_valid(root, i32::MIN, i32::MAX);
    let mut check = ValidBST::new(i32::MIN);
    return check.inorder_recursive_traversal_approach(root);
}

pub fn check_valid(root: Option<Rc<RefCell<TreeNode>>>, min: i32, max: i32) -> bool {
    let mut check_left = true;
    let mut check_right = true;

    match root {
        Some(the_root) => {
            println!("what the root is {:#?} **********", the_root.borrow().val);

            if the_root.borrow().val <= min || the_root.borrow().val >= max {
                return false;
            } else {
                println!("did it make it here on the left ###########");
                if the_root.borrow().left.is_some() {
                    check_left = check_valid(
                        Some(Rc::clone(the_root.borrow().left.as_ref().unwrap())),
                        min,
                        the_root.borrow().val,
                    );
                }

                println!("did it make it here on the RIGHT-----");

                if the_root.borrow().right.is_some() {
                    check_right = check_valid(
                        Some(Rc::clone(the_root.borrow().right.as_ref().unwrap())),
                        the_root.borrow().val,
                        max,
                    )
                }
            }
        }
        None => {}
    }

    return check_left && check_right;
}

struct ValidBST {
    prev: i32,
}

impl ValidBST {
    pub fn new(prev: i32) -> Self {
        ValidBST { prev }
    }

    pub fn inorder_recursive_traversal_approach(
        &mut self,
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return true;
        }
        match root {
            Some(the_root) => {
                match &the_root.borrow().left {
                    Some(the_left) => {
                        if !self.inorder_recursive_traversal_approach(Some(Rc::clone(&the_left))) {
                            return false;
                        }
                    }
                    None => {}
                }
                if the_root.borrow().val <= self.prev {
                    return false;
                }
                self.prev = the_root.borrow().val;

                match &the_root.borrow().right {
                    Some(the_right) => {
                        return self
                            .inorder_recursive_traversal_approach(Some(Rc::clone(&the_right)));
                    }
                    None => {}
                }
            }
            None => {}
        }
        return true;
    }

    pub fn inorder_iterative_traversal_approach(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut stack = vec![];
        let mut prev = i32::MIN;

        let mut root = Some(Rc::clone(&root.as_ref().unwrap()));

        while !stack.is_empty() || root.as_ref().is_some() {
            while root.is_some() {
                stack.push(Some(Rc::clone(&root.as_ref().unwrap())));
                // root = Some(Rc::clone)
            }
        }

        return true;
    }
}

#[cfg(test)]
mod test_validate_bst_mod {
    use crate::recursion::create_bst::{make_bst, TreeNode};
    use crate::recursion::validate_bst::is_valid_bst;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_validate_possible_bst() {
        // let list = vec![4, 1];

        // let bst = make_bst(list);

        // assert_eq!(is_valid_bst(bst), true);
        // assert_eq!(is_valid_bst(None), true);
        let bst = make_bst(vec![1, 1]);
        assert_eq!(is_valid_bst(bst), false);
    }

    #[test]
    fn test_invalid_bst() {
        let the_node = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        the_node.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(10))));

        assert_eq!(is_valid_bst(the_node), false);
    }

    #[test]
    fn test_valid_bst() {
        let the_node = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        the_node.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(is_valid_bst(the_node), false);

        // let the_node = Some(Rc::new(RefCell::new(TreeNode::new(2))));

        // the_node.as_ref().unwrap().borrow_mut().left =
        //     Some(Rc::new(RefCell::new(TreeNode::new(1))));

        // the_node.as_ref().unwrap().borrow_mut().right =
        //     Some(Rc::new(RefCell::new(TreeNode::new(3))));

        // assert_eq!(is_valid_bst(the_node), true);
    }

    #[test]
    fn test_multi_child() {
        let the_node = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        the_node.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(4))));

        the_node.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let right_node = Some(Rc::new(RefCell::new(TreeNode::new(6))));

        right_node.as_ref().unwrap().borrow_mut().left =
            Some(Rc::new(RefCell::new(TreeNode::new(3))));

        right_node.as_ref().unwrap().borrow_mut().right =
            Some(Rc::new(RefCell::new(TreeNode::new(7))));

        the_node.as_ref().unwrap().borrow_mut().right =
            Some(Rc::clone(right_node.as_ref().unwrap()));

        // println!("what the node looks like {:#?}", the_node);

        assert_eq!(is_valid_bst(the_node), false);
    }

    #[test]
    fn test_single_node() {
        let the_node = Some(Rc::new(RefCell::new(TreeNode::new(2147483647))));

        assert_eq!(is_valid_bst(the_node), true);
    }

    #[test]
    fn test_minimum_possible_node() {
        let the_node = Some(Rc::new(RefCell::new(TreeNode::new(-2147483648))));

        assert_eq!(is_valid_bst(the_node), true);
    }
}
