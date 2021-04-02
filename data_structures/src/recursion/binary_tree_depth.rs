pub mod binary_tree_depth {

    use std::collections::VecDeque;
    use std::rc::Rc;

    // Definition for a binary tree node.
    use crate::recursion::create_bst::NodeT;

    #[derive(Debug, PartialEq, Eq)]
    pub struct TreeNode {
        pub val: i32,
        pub left: NodeT,
        pub right: NodeT,
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

    pub fn max_depth_recursion(root: NodeT) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut left: NodeT = None;
        if root.as_ref().unwrap().borrow().left.is_some() {
            left = Some(Rc::clone(
                &root.as_ref().unwrap().borrow().left.as_ref().unwrap(),
            ));
        }

        let left_result = max_depth_recursion(left);

        let mut right: NodeT = None;

        if root.as_ref().unwrap().borrow().right.is_some() {
            right = Some(Rc::clone(
                &root.as_ref().unwrap().borrow().right.as_ref().unwrap(),
            ))
        }

        let right_result = max_depth_recursion(right);

        return std::cmp::max(left_result, right_result) + 1;
    }

    pub fn bfs_approach(root: NodeT) -> i32 {
        let mut queue: VecDeque<NodeT> = VecDeque::new();
        let mut depth = 0;

        if root.is_some() {
            queue.push_back(Some(Rc::clone(&root.as_ref().unwrap())));

            while !queue.is_empty() {
                let queue_length = queue.len();
                depth += 1;

                for _index in 0..queue_length {
                    let curr = queue.pop_front().unwrap();

                    if curr.is_some() && curr.as_ref().unwrap().borrow().left.is_some() {
                        queue.push_back(Some(Rc::clone(
                            &curr.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                        )));
                    }

                    if curr.is_some() && curr.as_ref().unwrap().borrow().right.is_some() {
                        queue.push_back(Some(Rc::clone(
                            &curr.as_ref().unwrap().borrow().right.as_ref().unwrap(),
                        )));
                    }
                }
            }
        }

        return depth;
    }
}

#[cfg(test)]
mod test_max_depth_mod {
    use crate::recursion::binary_tree_depth::binary_tree_depth::{
        bfs_approach, max_depth_recursion,
    };
    use crate::recursion::create_bst::make_bst;

    #[test]
    fn test_max_depth_bfs() {
        let list = vec![1, 2, 3, 4, 5, 6, 7];
        let bst = make_bst(list);

        assert_eq!(bfs_approach(bst), 3);

        let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let bst = make_bst(list);
        assert_eq!(bfs_approach(bst), 4);
    }

    #[test]
    fn test_max_depth_recursion() {
        let list = vec![1, 2, 3, 4, 5, 6, 7];
        let bst = make_bst(list);

        assert_eq!(max_depth_recursion(bst), 3);

        let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let bst = make_bst(list);
        assert_eq!(max_depth_recursion(bst), 4);
    }
}
