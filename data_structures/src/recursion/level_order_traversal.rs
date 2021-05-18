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
                            let the_node = the_current.as_ref().borrow();

                            if let Some(left) = &the_node.left {
                                queue.push_back(Rc::clone(left));
                            }

                            if let Some(right) = &the_node.right {
                                queue.push_back(Rc::clone(right));
                            }

                            current_level.push(the_node.val);
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

pub fn recursive_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    type Node = Rc<RefCell<TreeNode>>;

    let mut output: Vec<Vec<i32>> = vec![];

    fn traverse(node: &Node, output: &mut Vec<Vec<i32>>, depth: usize) {
        let n = node.borrow();

        if output.len() == depth {
            output.push(Vec::new())
        }

        output[depth].push(n.val);

        if let Some(l) = &n.left {
            traverse(&l, output, depth + 1)
        }

        if let Some(r) = &n.right {
            traverse(&r, output, depth + 1)
        }
    }

    if let Some(root) = root {
        traverse(&root, &mut output, 0);
    }

    output
}
