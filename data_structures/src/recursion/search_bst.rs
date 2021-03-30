use crate::recursion::create_bst::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct SearchBST {
    root: Option<Rc<RefCell<TreeNode>>>,
}

impl SearchBST {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn search_bst(&mut self, root: Option<Rc<RefCell<TreeNode>>>, val: i32) {
        match root {
            Some(ref the_root) => {
                if the_root.as_ref().borrow().val == val {
                    self.root = Some(Rc::clone(&the_root));

                    return;
                }
                if the_root.as_ref().borrow().left.is_some() {
                    self.search_bst(
                        Some(Rc::clone(
                            &the_root.as_ref().borrow().left.as_ref().unwrap(),
                        )),
                        val,
                    );
                }
                if the_root.as_ref().borrow().right.is_some() {
                    self.search_bst(
                        Some(Rc::clone(
                            &the_root.as_ref().borrow().right.as_ref().unwrap(),
                        )),
                        val,
                    );
                }
            }
            None => {
                return;
            }
        }
        return;
    }
}

pub fn recursive_search(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    let mut result = SearchBST::new();

    result.search_bst(root, val);

    return result.root;
}

#[cfg(test)]
mod test_search_bst_cont {
    #[test]
    fn test_search_bst() {
        use crate::recursion::{create_bst::make_bst, search_bst::recursive_search};

        let input = make_bst(vec![1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(recursive_search(input, 2), make_bst(vec![2, 4, 5]));
    }
}
