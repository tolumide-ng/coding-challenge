/// For this question, you will write a program that, given a positive integer M and a list of
/// integers L, outputs the list element M links away from the end of the list. For this program,
/// we will use 1-indexing. That means mth_to_last(1) is the "1st-to-last" element, or simply
/// the last element in the list.
use super::linked_rcs::LinkedList;
use super::node::{Node, NodeOption};

use std::rc::Rc;

pub fn get_nth_elem(index: usize, list: Vec<String>) -> NodeOption {
    let mut linked_list = LinkedList::new_empty();

    for value in list {
        linked_list.append_end(value);
    }

    return linked_list.get_pointer(index);
}

impl LinkedList {
    // POINTER METHOD TO GET THE NTH ELEMENT FROM THE LAST

    fn get_pointer(&mut self, index: usize) -> NodeOption {
        match &self.head {
            Some(head) => {
                let mut current_pointer = Some(Rc::clone(head));
                let mut target_elem: NodeOption = None;
                let mut watcher = 1;
                if current_pointer.is_some() {
                    while current_pointer.is_some() {
                        let mut next_pointer: NodeOption = None;
                        match current_pointer {
                            Some(ref pointer) => {
                                if pointer.borrow().next.is_some() {
                                    next_pointer =
                                        Some(Rc::clone(pointer.borrow().next.as_ref().unwrap()))
                                }
                            }
                            None => {}
                        }
                        current_pointer = next_pointer;
                        if watcher == index {
                            target_elem = Some(Rc::clone(head));
                        }
                        if watcher > index {
                            let next_pointer: NodeOption = Some(Rc::clone(
                                target_elem
                                    .as_ref()
                                    .unwrap()
                                    .borrow()
                                    .next
                                    .as_ref()
                                    .unwrap(),
                            ));
                            target_elem = next_pointer;
                        }
                        watcher += 1;
                    }
                }
                if target_elem.is_some() {
                    return target_elem;
                }
            }
            None => {}
        }

        return None;
    }
}

mod test_nth_pointer {
    use super::*;

    #[test]
    fn get_nth_elem_from_end() {
        let result = get_nth_elem(
            4,
            vec![
                String::from("A"),
                String::from("B"),
                String::from("C"),
                String::from("D"),
                String::from("E"),
            ],
        );

        assert!(result.is_some());
        assert_eq!(result.unwrap().borrow().data, "B".to_string());
    }

    #[test]
    fn get_nth_elem_empty() {
        let result = get_nth_elem(3, vec![]);

        assert!(result.is_none());
    }

    #[test]
    fn get_not_enough_nth_elem() {
        let result = get_nth_elem(3, vec!["A".to_string()]);

        assert!(result.is_none());
    }
}
