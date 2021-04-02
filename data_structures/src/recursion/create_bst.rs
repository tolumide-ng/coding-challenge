use std::collections::VecDeque;

pub type NodeT = Option<Rc<RefCell<TreeNode>>>;

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

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct NodeTimes {
    node: NodeT,
    times: i32,
}

impl NodeTimes {
    pub fn new(val: i32) -> Self {
        Self {
            node: Some(Rc::new(RefCell::new(TreeNode::new(val)))),
            times: 0,
        }
    }

    pub fn update(&mut self) {
        self.times += 1;
    }
}

pub fn make_bst(list: Vec<i32>) -> NodeT {
    let mut queue: VecDeque<NodeTimes> = VecDeque::new();

    if list.is_empty() {
        return None;
    }

    let the_node = NodeTimes::new(list[0]);
    let head = Some(Rc::clone(&the_node.node.as_ref().unwrap()));

    queue.push_back(the_node);

    if list.len() == 1 {
        return head;
    }

    // queue.push_back(node);

    // head = Some(Rc::clone(&node.node.unwrap()));

    // let mut bt = BinaryHeap::new(15);
    for index in 1..list.len() {
        if !queue.is_empty() {
            let mut parent_node = queue.get_mut(0).unwrap();
            let new_node = NodeTimes::new(list[index]);
            if parent_node.times == 0 {
                parent_node.node.as_ref().unwrap().borrow_mut().left =
                    Some(Rc::clone(&new_node.node.as_ref().unwrap()));
            }
            if parent_node.times == 1 {
                parent_node.node.as_ref().unwrap().borrow_mut().right =
                    Some(Rc::clone(&new_node.node.as_ref().unwrap()));
            }

            parent_node.times += 1;
            if parent_node.times == 2 {
                queue.pop_front();
            }

            queue.push_back(new_node);
        }
    }

    return head;
}

#[cfg(test)]
mod test_create_bst_mod {
    #[test]
    fn test_creates_bst() {
        use super::*;

        let list = vec![1, 2, 3, 4, 5, 6, 7, 8];

        let bst = make_bst(list);

        match bst {
            Some(the_bst) => {
                assert_eq!(the_bst.as_ref().borrow().val, 1);

                match the_bst.borrow().left {
                    Some(ref left_bst) => {
                        assert_eq!(left_bst.as_ref().borrow().val, 2);

                        match left_bst.borrow().left {
                            Some(ref left_bst) => {
                                assert_eq!(left_bst.as_ref().borrow().val, 4);

                                match left_bst.borrow().left {
                                    Some(ref left_bst) => {
                                        assert_eq!(left_bst.as_ref().borrow().val, 8);

                                        assert!(left_bst.borrow().left.is_none());
                                    }
                                    None => {}
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }

                match the_bst.borrow().right {
                    Some(ref right_bst) => {
                        assert_eq!(right_bst.as_ref().borrow().val, 3);

                        match right_bst.borrow().left {
                            Some(ref left_bst) => {
                                assert_eq!(left_bst.as_ref().borrow().val, 6);
                            }
                            None => {}
                        }

                        match right_bst.borrow().right {
                            Some(ref right_bst) => {
                                assert_eq!(right_bst.as_ref().borrow().val, 7);

                                assert!(right_bst.borrow().left.is_none());
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}
