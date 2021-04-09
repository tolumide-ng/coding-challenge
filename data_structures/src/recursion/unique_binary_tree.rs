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
struct Solution {
    // since constraint already state that n is 1 <= n <= 8, we can caceh a 2,3 tree as either 23 or 32
    cache: HashMap<String, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    n: i32,
}

pub fn gen_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut dd = Solution::new(n);
    return dd.generate_trees(n);
}

impl Solution {
    pub fn new(n: i32) -> Self {
        Self {
            cache: HashMap::new(),
            n,
        }
    }

    pub fn gen_node(&self, n: i32) -> Option<Rc<RefCell<TreeNode>>> {
        return Some(Rc::new(RefCell::new(TreeNode::new(n))));
    }

    pub fn generate_trees(&mut self, n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n == 0 {
            return vec![];
        }

        let children: Vec<i32> = (1..=n).collect();
        return self.trees_helper(&children);
    }

    pub fn trees_helper(&mut self, children: &Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut possible_trees: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        if children.len() == 0 {
            return possible_trees;
        }

        if children.len() == 1 {
            possible_trees.push(self.gen_node(children[0]));
            return possible_trees;
        }

        if children.len() == 2 {
            let mut child_one = children[0];
            let mut child_two = children[1];

            let mut key = String::from(children[0].to_string());
            key.push_str(&children[1].to_string());

            if self.cache.get(&key).is_some() {
                return (*self.cache.get(&key).as_ref().unwrap()).to_vec();
            }

            if children[0] > children[1] {
                child_one = children[1];
                child_two = children[0];
            }

            let first_tree = self.gen_node(child_two);
            first_tree.as_ref().unwrap().borrow_mut().left = self.gen_node(child_one);

            possible_trees.push(first_tree);

            let second_tree = self.gen_node(child_one);
            second_tree.as_ref().unwrap().borrow_mut().right = self.gen_node(child_two);

            possible_trees.push(second_tree);

            self.cache.insert(key, possible_trees.clone());

            return possible_trees;
        }

        for index in 0..children.len() {
            let current_value = &children[index];

            let mut left_children = vec![0; (index) as usize];
            left_children.copy_from_slice(&children[0..index]);

            let diff = children.len() - index - 1;

            let mut right_children = vec![0; diff];

            right_children.copy_from_slice(&children[index + 1..]);

            let left_trees = self.trees_helper(&left_children);
            let right_trees = self.trees_helper(&right_children);

            for left_index in 0..left_trees.len() {
                for right_index in 0..right_trees.len() {
                    let new_node = self.gen_node(*current_value);
                    new_node.as_ref().unwrap().borrow_mut().left =
                        Some(Rc::clone(&left_trees[left_index].as_ref().unwrap()));
                    new_node.as_ref().unwrap().borrow_mut().right =
                        Some(Rc::clone(&right_trees[right_index].as_ref().unwrap()));
                    possible_trees.push(new_node);
                }
            }

            if left_trees.len() == 0 && right_trees.len() > 0 {
                for right_index in 0..right_trees.len() {
                    let new_node = self.gen_node(*current_value);
                    new_node.as_ref().unwrap().borrow_mut().right =
                        Some(Rc::clone(&right_trees[right_index].as_ref().unwrap()));
                    possible_trees.push(new_node);
                }
            }

            if right_trees.len() == 0 && left_trees.len() > 0 {
                for left_index in 0..left_trees.len() {
                    let new_node = self.gen_node(*current_value);
                    new_node.as_ref().unwrap().borrow_mut().right =
                        Some(Rc::clone(&left_trees[left_index].as_ref().unwrap()));
                    possible_trees.push(new_node);
                }
            }
        }

        return possible_trees;
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
        assert_eq!(gen_trees(1).len(), 1);
        gen_trees(8);
        // assert_eq!(3, 4);
    }
}
