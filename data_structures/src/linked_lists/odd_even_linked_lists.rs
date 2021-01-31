/// LEETCODE: https://leetcode.com/explore/learn/card/linked-list/219/classic-problems/1208/
use super::linked_rcs::LinkedList;
use super::node::NodeOption;

use std::rc::Rc;

impl LinkedList {
    /// ARRANGE ODD EVEN ELEMENTS ON LINKED LIST:
    /// Time complexity - O(n)
    /// Space complexity - O(1)
    pub fn arrange_odd_even_nodes(&mut self) {
        if self.head.is_some() {
            let mut curr_node = Some(Rc::clone(self.head.as_ref().unwrap()));
            let mut last_odd_node = Some(Rc::clone(self.head.as_ref().unwrap()));
            let mut last_even_node: NodeOption = None;
            let mut index = 0;

            while curr_node.as_ref().is_some() {
                index += 1;

                if index % 2 != 0 && index != 1 {
                    let mut the_next_node: NodeOption = None;

                    match curr_node.as_ref().unwrap().borrow().next {
                        Some(ref next_neigbour) => {
                            the_next_node = Some(Rc::clone(next_neigbour));
                        }
                        None => {}
                    }

                    if last_even_node.as_ref().is_some() {
                        let next_node = Rc::clone(
                            last_even_node
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .next
                                .as_ref()
                                .unwrap(),
                        );
                        last_even_node = Some(next_node);
                    } else {
                        last_even_node = Some(Rc::clone(
                            last_odd_node
                                .as_ref()
                                .unwrap()
                                .borrow()
                                .next
                                .as_ref()
                                .unwrap(),
                        ));
                    }

                    let first_even_node = Some(Rc::clone(
                        last_odd_node
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .next
                            .as_ref()
                            .unwrap(),
                    ));

                    last_odd_node.as_ref().unwrap().borrow_mut().next =
                        Some(Rc::clone(curr_node.as_ref().unwrap()));

                    let new_odd = Rc::clone(curr_node.as_ref().unwrap());

                    last_odd_node = Some(new_odd);

                    if the_next_node.is_some() {
                        last_even_node.as_ref().unwrap().borrow_mut().next =
                            Some(Rc::clone(the_next_node.as_ref().unwrap()));
                    } else {
                        last_even_node.as_ref().unwrap().borrow_mut().next = None;
                    }

                    if first_even_node.as_ref().is_some() {
                        last_odd_node.as_ref().unwrap().borrow_mut().next =
                            Some(Rc::clone(first_even_node.as_ref().unwrap()));
                    } else {
                        last_odd_node.as_ref().unwrap().borrow_mut().next = None;
                    }
                    curr_node = the_next_node;
                } else {
                    let mut next_node: NodeOption = None;

                    match curr_node.as_ref().unwrap().borrow().next {
                        Some(ref the_node) => {
                            next_node = Some(Rc::clone(the_node));
                        }
                        None => {}
                    }

                    curr_node = next_node;
                }
            }

            // self.tail.unwrap()
            if self.tail.as_ref().is_some() {
                let mut the_tail = Some(Rc::clone(self.tail.as_ref().unwrap()));

                while the_tail.as_ref().unwrap().borrow().next.as_ref().is_some() {
                    let next_value =
                        Rc::clone(the_tail.as_ref().unwrap().borrow().next.as_ref().unwrap());
                    the_tail = Some(Rc::clone(&next_value));
                }

                self.tail = the_tail;
            }
        }
    }
}

#[test]
fn odd_even_linked_lists() {
    let mut list = LinkedList::new_empty();

    list.append_end("node_1".to_string());
    list.append_end("node_2".to_string());
    list.append_end("node_3".to_string());
    list.append_end("node_4".to_string());
    list.append_end("node_5".to_string());
    list.append_end("node_6".to_string());
    list.append_end("node_7".to_string());
    list.append_end("node_8".to_string());
    list.append_end("node_9".to_string());

    list.arrange_odd_even_nodes();

    match list.tail {
        Some(ref tail) => {
            assert_eq!(tail.borrow().data, "node_8");
            assert_eq!(tail.borrow().next, None);
        }
        None => {}
    }

    match list.head {
        Some(ref head) => {
            assert_eq!(head.borrow().data, "node_1");

            match head.borrow().next {
                Some(ref next_head) => {
                    assert_eq!(next_head.borrow().data, "node_3");

                    match next_head.borrow().next {
                        Some(ref newer_head) => {
                            assert_eq!(newer_head.borrow().data, "node_5");
                        }
                        None => {}
                    }
                }
                None => {}
            }
        }
        None => {}
    };
}
