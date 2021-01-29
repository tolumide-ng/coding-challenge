use super::linked_rcs::LinkedList;
use super::node::{NodeOption, NodeRef};

use std::rc::Rc;

impl LinkedList {
    pub fn reverse_the_list(&mut self, head: NodeOption) -> NodeOption {
        if head.as_ref().is_none() || head.as_ref().unwrap().as_ref().borrow().next.is_none() {
            return head;
        }

        let mut next_head: NodeOption = None;
        match head {
            Some(ref de_head) => match de_head.borrow().next {
                Some(ref new_head) => {
                    next_head = Some(Rc::clone(new_head));
                }

                None => {}
            },
            None => {}
        };

        let result = self.reverse_the_list(next_head);

        let end_result = match result {
            Some(ref new_result) => match head {
                Some(ref de_head) => {
                    de_head.borrow_mut().next = None;

                    let mut new_result_tail = Some(Rc::clone(new_result));

                    while new_result_tail.as_ref().unwrap().borrow().next.is_some() {
                        new_result_tail = Some(Rc::clone(
                            new_result_tail.unwrap().borrow().next.as_ref().unwrap(),
                        ));
                    }

                    match new_result_tail {
                        Some(ref mut new_tail) => {
                            new_tail.borrow_mut().next = Some(Rc::clone(de_head));
                        }
                        None => {}
                    }

                    self.head = Some(Rc::clone(new_result));
                    self.tail = head;
                    return Some(Rc::clone(new_result));
                }
                None => None,
            },
            None => None,
        };

        return end_result;
    }
}

#[test]
fn premo_wayne() {
    // use super::*;

    let mut list = LinkedList::new_empty();
    list.append_end("node_1".to_string());
    list.append_end("node_2".to_string());
    list.append_end("node_3".to_string());
    list.append_end("node_4".to_string());
    list.append_end("node_5".to_string());

    list.reverse_the_list(Some(Rc::clone(list.head.as_ref().unwrap())));

    match list.head {
        Some(ref head) => {
            assert_eq!(head.borrow().data, "node_5");

            match head.borrow().next {
                Some(ref next_head) => {
                    assert_eq!(next_head.borrow().data, "node_4");
                }
                None => {}
            }
        }
        None => {}
    };

    match list.tail {
        Some(ref tail) => {
            assert_eq!(tail.borrow().data, "node_1");
            assert_eq!(tail.borrow().next, None);
        }
        None => {}
    }
}
