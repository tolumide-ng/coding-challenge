use std::cell::RefCell;
use std::rc::Rc;

type BNode<T> = Option<Rc<RefCell<TreeNode<T>>>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T: Debug + Copy + Clone + Eq + PartialEq> {
    pub val: T,
    pub left: BNode<T>,
    pub right: BNode<T>,
}

impl<T: Debug + Copy + Clone + Eq + PartialEq> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::fmt::Debug;

pub fn build_tree<T: Debug + Copy + Clone + Eq + PartialEq>(
    preorder: Vec<T>,
    inorder: Vec<T>,
) -> BNode<T> {
    let preorder_len = preorder.len();
    let inorder_len = inorder.len();

    return recursive_build_tree(&preorder[0..preorder_len], &inorder[0..inorder_len]);
}

pub fn recursive_build_tree<T: Debug + Copy + Clone + Eq + PartialEq>(
    preorder: &[T],
    inorder: &[T],
) -> BNode<T> {
    if inorder.len() <= 0 {
        return None;
    }

    let root_index = preorder.iter().position(|&s| inorder.contains(&s)).unwrap();

    let root = preorder[root_index];
    let mut the_root = TreeNode::new(root);

    let inorder_index = inorder.iter().position(|&s| s == root).unwrap();

    let left = &inorder[0..inorder_index];

    let mut right: &[T] = &[];

    if inorder_index + 1 < inorder.len() {
        right = &inorder[inorder_index + 1..];
    };

    let the_left = recursive_build_tree(preorder, left);
    let the_right = recursive_build_tree(preorder, right);

    the_root.left = the_left;
    the_root.right = the_right;

    return Some(Rc::new(RefCell::new(the_root)));
}

#[cfg(test)]
mod test_preorder_inorder {
    use super::build_tree;

    #[test]
    fn test_pre_in_list() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        let tree = build_tree(preorder, inorder);

        assert_eq!(tree.as_ref().unwrap().borrow().val, 3);
        assert_eq!(
            tree.as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .val,
            9
        );
        assert_eq!(
            tree.as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .as_ref()
                .borrow()
                .val,
            20
        );
    }
}
