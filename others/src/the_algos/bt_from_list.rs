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

fn build_tree<T: Debug + Copy + Clone + Eq + PartialEq>(
    inorder: Vec<T>,
    postorder: Vec<T>,
) -> BNode<T> {
    let inorder_len = inorder.len();
    let postorder_len = postorder.len();

    return recursive_build_tree(&inorder[0..inorder_len], &postorder[0..postorder_len]);
}

fn recursive_build_tree<T: Debug + Copy + Clone + Eq + PartialEq>(
    inorder: &[T],
    postorder: &[T],
) -> BNode<T> {
    if inorder.len() == 0 {
        return None;
    }

    let mut root_index = postorder
        .iter()
        .rev()
        .position(|&s| inorder.contains(&s))
        .unwrap();

    root_index = (postorder.len() - 1) - root_index;

    let inorder_index = inorder
        .iter()
        .position(|&s| s == postorder[root_index])
        .unwrap();

    let all_left = &inorder[0..inorder_index];
    // check first if +1 is not already bigger than the available index
    let mut all_right: &[T] = &[];

    if inorder_index + 1 < inorder.len() {
        all_right = &inorder[inorder_index + 1..];
    }

    let mut the_node = TreeNode::new(postorder[root_index]);

    let the_right = recursive_build_tree(all_right, postorder);

    let the_left = recursive_build_tree(all_left, postorder);

    the_node.left = the_left;
    the_node.right = the_right;

    return Some(Rc::new(RefCell::new(the_node)));
}

#[cfg(test)]
mod test_bt_from_list {
    use super::build_tree;

    #[test]
    fn test_bt_list() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];

        let tree = build_tree(inorder, postorder);

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
