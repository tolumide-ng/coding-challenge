use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::{Rc, Weak};

pub type DNodeRef<T> = Rc<RefCell<DNode<T>>>;
pub type DWeakNodeRef<T> = Weak<RefCell<DNode<T>>>;
pub type DNodeOption<T> = Option<DNodeRef<T>>;
pub type DWeakNodeOption<T> = Option<DWeakNodeRef<T>>;

#[derive(Debug)]
pub struct DNode<T: Debug + Display> {
    pub data: T,
    pub next: DNodeOption<T>,
    pub prev: DWeakNodeOption<T>,
}

impl<T> DNode<T>
where
    T: Display + Debug,
{
    pub fn new(data: T) -> DNodeRef<T> {
        Rc::new(RefCell::new(DNode {
            data,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Drop for DNode<T>
where
    T: Debug + Display,
{
    fn drop(&mut self) {
        println!("DNode with this data -> '{}' just dropped", self.data);
    }
}

pub struct ListDNodeIterator<T: Debug + Display> {
    pub current: DNodeOption<T>,
}

impl<T> ListDNodeIterator<T>
where
    T: Debug + Display,
{
    pub fn new(start_at: DNodeOption<T>) -> Self {
        ListDNodeIterator { current: start_at }
    }
}

impl<T> Iterator for ListDNodeIterator<T>
where
    T: Debug + Display,
{
    type Item = DNodeRef<T>;

    fn next(&mut self) -> DNodeOption<T> {
        let current = &self.current;

        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                result = Some(Rc::clone(current));
                match &current.borrow().next {
                    Some(next_node) => Some(Rc::clone(next_node)),
                    _ => None,
                }
            }
            _ => None,
        };

        return result;
    }
}

mod dtests {
    use super::*;

    #[test]
    fn test_new_dnode_string() {
        let node = DNode::new("node_1".to_string());

        assert_eq!(node.as_ref().borrow().data, "node_1".to_string());
    }

    #[test]
    fn test_new_dnode_integer() {
        let node = DNode::new(1);

        assert_eq!(node.as_ref().borrow().data, 1);
        assert!(node.as_ref().borrow().next.is_none());
    }
}
