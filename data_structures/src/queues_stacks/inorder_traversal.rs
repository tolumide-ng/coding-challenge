#[allow(unused_variables)]
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub mod inorder_traversal {
    // use std::borrow::Borrow;

    use super::*;

    #[derive(Debug)]
    pub struct Traversal {
        stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
        output: Vec<i32>,
    }

    impl Traversal {
        pub fn get_recursive_traversal(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
            match root.as_ref() {
                Some(the_root) => {
                    match the_root.as_ref().borrow().left.as_ref() {
                        Some(the_left) => {
                            self.get_recursive_traversal(Some(Rc::clone(the_left)));
                        }
                        None => {}
                    }
                    self.output.push(the_root.as_ref().borrow().val);
                }
                None => {}
            }
            match root.as_ref() {
                Some(the_root) => match the_root.as_ref().borrow().right.as_ref() {
                    Some(the_right) => {
                        self.get_recursive_traversal(Some(Rc::clone(the_right)));
                    }
                    None => {}
                },
                None => {}
            }
        }

        pub fn get_recursive_traversal_method_2(&mut self, root: Option<Rc<RefCell<TreeNode>>>) {
            if root.as_ref().is_some() {
                if root.as_ref().unwrap().as_ref().borrow().left.is_some() {
                    self.get_recursive_traversal_method_2(Some(Rc::clone(
                        root.as_ref()
                            .unwrap()
                            .as_ref()
                            .borrow()
                            .left
                            .as_ref()
                            .unwrap(),
                    )));
                }

                self.output
                    .push(root.as_ref().unwrap().as_ref().borrow().val);

                if root.as_ref().unwrap().as_ref().borrow().right.is_some() {
                    self.get_recursive_traversal_method_2(Some(Rc::clone(
                        root.as_ref()
                            .unwrap()
                            .as_ref()
                            .borrow()
                            .right
                            .as_ref()
                            .unwrap(),
                    )));
                }
            }
        }
    }
}
