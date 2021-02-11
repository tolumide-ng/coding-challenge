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

type Tree = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        return Solution::is_mirror(
            Some(Rc::clone(&root.as_ref().unwrap())),
            (Some(Rc::clone(&root.as_ref().unwrap()))),
        );
    }

    pub fn is_mirror(root_one: Tree, root_two: Tree) -> bool {
        if root_one.is_none() && root_two.is_none() {
            return true;
        }

        if (root_one.is_none() && root_two.is_some()) || (root_one.is_some() && root_two.is_none())
        {
            return false;
        }
        // let the_root_one = root_one.unwrap().replace(TreeNode::new(0));
        // let the_root_two = root_two.unwrap().replace(TreeNode::new(0));
        let the_root_one = Some(Rc::clone(&root_one.as_ref().unwrap()));
        let the_root_two = Some(Rc::clone(&root_two.as_ref().unwrap()));
        // as_ref().unwrap().borrow()
        let val_one = the_root_one.as_ref().unwrap().borrow();
        let val_two = the_root_two.as_ref().unwrap().borrow();
        // return the_root_one.val == the_root_two.val && Solution::is_mirror(the_root_one.right, the_root_two.left) && Solution::is_mirror(the_root_one.left, the_root_two.right);
        println!(
            "REASON WHY ITS NOT {:#?}, {:#?}",
            the_root_one.as_ref().unwrap().borrow().val,
            the_root_two.as_ref().unwrap().borrow().val
        );

        if the_root_one.as_ref().unwrap().borrow().val
            == the_root_two.as_ref().unwrap().borrow().val
        {
            let mut one_left: Tree = None;
            if root_one.as_ref().unwrap().borrow().left.is_some() {
                one_left = Some(Rc::clone(
                    &root_one.as_ref().unwrap().borrow().left.as_ref().unwrap(),
                ));
            }
            let mut one_right: Tree = None;
            if root_one.as_ref().unwrap().borrow().right.is_some() {
                one_right = Some(Rc::clone(
                    &root_one.as_ref().unwrap().borrow().right.as_ref().unwrap(),
                ));
            }
            let mut two_left: Tree = None;
            if the_root_two.as_ref().unwrap().borrow().left.is_some() {
                two_left = Some(Rc::clone(
                    &the_root_two
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap(),
                ));
            }
            let mut two_right: Tree = None;
            if the_root_two.as_ref().unwrap().borrow().right.is_some() {
                two_right = Some(Rc::clone(
                    &the_root_two
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .right
                        .as_ref()
                        .unwrap(),
                ))
            }
            return Solution::is_mirror(one_right, two_left)
                && Solution::is_mirror(one_left, two_right);
        }
        println!("why am i here???");
        return false;
    }
}
