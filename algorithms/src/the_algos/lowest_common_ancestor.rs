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

type TreeType = Option<Rc<RefCell<TreeNode>>>;
type TreeRcType = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
struct LCA {
    found_x: bool,
    found_y: bool,
}

use std::cell::RefCell;
use std::rc::Rc;

pub fn lowest_common_ancestor(root: TreeType, p: TreeType, q: TreeType) -> TreeType {
    let mut ancestor = LCA {
        found_x: false,
        found_y: false,
    };

    if root.is_none() || p.is_none() || q.is_none() {
        return None;
    }

    let the_ancestor = ancestor.find_ancestor(root, p.unwrap(), q.unwrap());
    if ancestor.found_x && ancestor.found_y {
        return the_ancestor;
    }

    return None;
}

impl LCA {
    fn find_ancestor(&mut self, root: TreeType, x: TreeRcType, y: TreeRcType) -> TreeType {
        if root.is_none() {
            return None;
        }

        let mut root_val: TreeType = None;
        let x_val = x.borrow().val;
        let y_val = y.borrow().val;
        let mut left_child: TreeType = None;
        let mut right_child: TreeType = None;

        match root.as_ref() {
            Some(the_root) => {
                if the_root.borrow().val == x_val {
                    root_val = Some(Rc::clone(&the_root));
                    self.found_x = true;
                }

                if the_root.borrow().val == y_val {
                    root_val = Some(Rc::clone(&the_root));
                    self.found_y = true;
                }

                match the_root.as_ref().borrow().left.as_ref() {
                    Some(the_left) => {
                        left_child = self.find_ancestor(
                            Some(Rc::clone(the_left)),
                            Rc::clone(&x),
                            Rc::clone(&y),
                        );
                    }
                    None => {}
                }

                match the_root.as_ref().borrow().right.as_ref() {
                    Some(the_right) => {
                        right_child = self.find_ancestor(
                            Some(Rc::clone(the_right)),
                            Rc::clone(&x),
                            Rc::clone(&y),
                        );
                    }
                    None => {}
                }
            }
            None => {}
        }

        if root_val.is_none() {
            if left_child.is_some() && right_child.is_none() {
                root_val = Some(Rc::clone(left_child.as_ref().unwrap()));
            }

            if right_child.is_some() && left_child.is_none() {
                root_val = Some(Rc::clone(right_child.as_ref().unwrap()));
            }

            if left_child.is_some() && right_child.is_some() {
                root_val = Some(Rc::clone(root.as_ref().unwrap()));
            }
        }

        return root_val;
    }
}

#[test]
fn test_find_ancestor() {
    let left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    let mut root = TreeNode::new(1);
    root.left = left;
    let mut root = Some(Rc::new(RefCell::new(root)));

    let x = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let y = Some(Rc::new(RefCell::new(TreeNode::new(2))));

    match lowest_common_ancestor(root, x, y) {
        Some(result) => {
            assert_eq!(result.borrow().val, 1);
        }
        None => {}
    }
}
