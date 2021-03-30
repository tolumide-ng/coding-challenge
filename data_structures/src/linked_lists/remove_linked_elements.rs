// Remove all elements from a linked list of integers that have value val.

use super::linked_rcs::LinkedList;
use super::node::NodeOption;
use std::rc::Rc;

impl LinkedList {
    /// REMOVE ELEMENTS FROM A LINKED LIST:
    /// Time complexity - O(n)
    /// Space complexity - O(1)

    pub fn remove_element(&mut self, target: String) {
        let mut prev_node: NodeOption = None;
        let mut curr_node = Some(Rc::clone(self.head.as_ref().unwrap()));

        while curr_node.is_some() {
            if curr_node.as_ref().unwrap().borrow().data != target {
                if curr_node.as_ref().unwrap().borrow().next.is_some() {
                    let the_node = Some(Rc::clone(curr_node.as_ref().unwrap()));

                    curr_node = Some(Rc::clone(
                        the_node.as_ref().unwrap().borrow().next.as_ref().unwrap(),
                    ));

                    prev_node = the_node;
                } else {
                    prev_node = curr_node;
                    curr_node = None;
                }
            } else {
                if prev_node.is_some() {
                    if curr_node.as_ref().unwrap().borrow().next.is_some() {
                        let next_node = Some(Rc::clone(curr_node.as_ref().unwrap()));

                        prev_node.as_ref().unwrap().borrow_mut().next = Some(Rc::clone(
                            next_node.as_ref().unwrap().borrow().next.as_ref().unwrap(),
                        ));

                        // previous node remains the same old one

                        // curr_node changes to the next node (neighbour)
                        curr_node = Some(Rc::clone(
                            next_node.as_ref().unwrap().borrow().next.as_ref().unwrap(),
                        ));
                    } else {
                        curr_node = None;
                        prev_node.as_ref().unwrap().borrow_mut().next = None;
                        self.tail = Some(Rc::clone(prev_node.as_ref().unwrap()));
                    }
                } else {
                    self.head = Some(Rc::clone(
                        curr_node.as_ref().unwrap().borrow().next.as_ref().unwrap(),
                    ));
                    curr_node = Some(Rc::clone(self.head.as_ref().unwrap()));

                    // prev_node = Some(Rc::clone(self.head.as_ref().unwrap()));
                    prev_node = None;
                }
            }
        }
    }
}

#[test]
fn remove_linked_elem() {
    let mut list = LinkedList::new_empty();

    list.append_end(String::from("node_6"));
    list.append_end(String::from("node_1"));

    match list.head {
        Some(ref head) => {
            assert_eq!(head.borrow().data, "node_6");

            match head.borrow().next {
                Some(ref next_head) => {
                    assert_eq!(next_head.borrow().data, "node_1");
                    assert_eq!(next_head.borrow().next, None);
                }
                None => {}
            }
        }
        None => {}
    }
    list.append_end(String::from("node_2"));
    list.append_end(String::from("node_3"));
    list.append_end(String::from("node_4"));
    list.append_end(String::from("node_2"));
    list.append_end(String::from("node_5"));
    list.append_end(String::from("node_3"));
    list.append_end(String::from("node_6"));
    list.append_end(String::from("node_5"));

    list.remove_element("node_1".to_string());

    match list.head {
        Some(ref head) => {
            assert_eq!(head.borrow().data, "node_6");

            match head.borrow().next {
                Some(ref next_head) => {
                    assert_eq!(next_head.borrow().data, "node_2");
                    assert!(next_head.borrow().next.is_some());
                }
                None => {}
            }
        }
        None => {}
    }

    println!("THE LIST {:#?}", list);

    match list.tail {
        Some(ref tail) => {
            assert_eq!(tail.borrow().data, "node_5");
        }
        None => {}
    }
}
