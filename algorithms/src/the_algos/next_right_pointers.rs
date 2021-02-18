use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

type BNode<T> = Option<Rc<RefCell<TreeNode<T>>>>;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T: Debug + Copy + Clone + Eq + PartialEq> {
    pub val: T,
    pub left: BNode<T>,
    pub right: BNode<T>,
    pub next: BNode<T>,
}

pub fn connect_node() {}
