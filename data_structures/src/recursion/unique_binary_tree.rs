use std::collections::HashMap;

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

#[derive(Debug)]
struct UniqueBst {
    // since constraint already state that n is 1 <= n <= 8, we can caceh a 2,3 tree as either 23 or 32
    cache: HashMap<String, TreeVec>,
    n: i32,
}

pub fn gen_trees(n: i32) -> TreeVec {
    let mut dd = UniqueBst::new(n);
    return dd.generate_trees(n);
}

type TreeVec = Vec<Option<Rc<RefCell<TreeNode>>>>;

impl UniqueBst {
    pub fn new(n: i32) -> Self {
        Self {
            cache: HashMap::new(),
            n,
        }
    }

    pub fn gen_node(&self, n: i32) -> Option<Rc<RefCell<TreeNode>>> {
        return Some(Rc::new(RefCell::new(TreeNode::new(n))));
    }

    pub fn generate_trees(&mut self, n: i32) -> TreeVec {
        if n == 0 {
            return vec![];
        }

        // let children: Vec<i32> = (1..=n).collect();
        return self.trees_helper(1, n as usize);
    }

    pub fn trees_helper(&mut self, start: usize, end: usize) -> TreeVec {
        let mut all_trees: TreeVec = vec![];

        if start > end {
            all_trees.push(None);
            return all_trees;
        }

        for index in start..=end {
            let left = self.trees_helper(start, index - 1);
            let right = self.trees_helper(index + 1, end);

            for left_index in 0..left.len() {
                for right_index in 0..right.len() {
                    let new_node = self.gen_node(index as i32);
                    if left[left_index].is_some() {
                        new_node.as_ref().unwrap().borrow_mut().left =
                            Some(Rc::clone(&left[left_index].as_ref().unwrap()));
                    }

                    if right[right_index].is_some() {
                        new_node.as_ref().unwrap().borrow_mut().right =
                            Some(Rc::clone(&right[right_index].as_ref().unwrap()));
                    }

                    all_trees.push(new_node);
                }
            }
        }

        return all_trees;
    }
}

#[cfg(test)]
mod test_unique_bsts_mod {
    use crate::recursion::unique_binary_tree::gen_trees;

    #[test]
    fn test_unique_bsts() {
        assert_eq!(gen_trees(2).len(), 2);
        assert_eq!(gen_trees(3).len(), 5);
        assert_eq!(gen_trees(4).len(), 14);
        assert_eq!(gen_trees(5).len(), 42);
        assert_eq!(gen_trees(8).len(), 1430);
        assert_eq!(gen_trees(1).len(), 1);
    }
}
