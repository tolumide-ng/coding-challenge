use super::linked_rcs::LinkedList;
use super::node::{Node, NodeOption};
use std::rc::Rc;

impl LinkedList {
    pub fn is_linked_palindrome(&mut self) -> bool {
        let mut is_palindrome = false;

        if self.head.as_ref().is_some() && self.head.as_ref().unwrap().borrow().next.is_some() {
            let mut penultimate_ptr: NodeOption = None;
            let original_list = Some(Rc::clone(self.head.as_ref().unwrap()));
            let mut fast_ptr = Some(Rc::clone(self.head.as_ref().unwrap()));
            let mut slow_ptr = Some(Rc::clone(self.head.as_ref().unwrap()));

            while fast_ptr.as_ref().is_some() && fast_ptr.as_ref().unwrap().borrow().next.is_some()
            {
                let next_slow = Some(Rc::clone(
                    slow_ptr.as_ref().unwrap().borrow().next.as_ref().unwrap(),
                ));

                penultimate_ptr = slow_ptr;

                slow_ptr = next_slow;

                let mut next_fast: NodeOption = None;

                match fast_ptr {
                    Some(ref fast_val) => match fast_val.borrow().next.as_ref() {
                        Some(fast_next) => match fast_next.borrow().next.as_ref() {
                            Some(ref new_fast) => next_fast = Some(Rc::clone(&new_fast)),
                            None => {}
                        },
                        None => {}
                    },
                    None => {}
                }

                fast_ptr = next_fast;
            }

            // Palindromes can be odd or even length
            // Example: Odd length palindrome: "MADAM"
            // Example: Even length palindrome "123321"
            // We're dealing with an odd length palindrome

            // handle edge case for only two items in the linked list
            if slow_ptr.as_ref().is_some()
                && slow_ptr.as_ref().unwrap().borrow().next.as_ref().is_some()
            {
                if fast_ptr.as_ref().is_some() {
                    let value_in_slow = &slow_ptr.as_ref().unwrap().borrow().data;
                    let new_node = Node::new(value_in_slow.to_string());
                    penultimate_ptr.as_ref().unwrap().borrow_mut().next =
                        Some(Rc::clone(&new_node));

                    match penultimate_ptr.as_ref().unwrap().borrow().next {
                        Some(ref new_penultimate) => {
                            new_penultimate.borrow_mut().next =
                                Some(Rc::clone(slow_ptr.as_ref().unwrap()));
                        }
                        None => {}
                    }
                }

                self.reverse_the_list(Some(Rc::clone(slow_ptr.as_ref().unwrap())));

                // if fast_ptr.as_ref().is_some() {
                let mut ptr_to_append_to = Some(Rc::clone(penultimate_ptr.as_ref().unwrap()));

                if fast_ptr.as_ref().is_some() {
                    match penultimate_ptr {
                        Some(ref curr_pen) => match curr_pen.as_ref().borrow().next {
                            Some(ref next_pen) => {
                                ptr_to_append_to = Some(Rc::clone(next_pen));
                            }
                            None => {}
                        },
                        None => {}
                    }
                }

                let reversed_list = self.head.take();

                ptr_to_append_to.as_ref().unwrap().borrow_mut().next =
                    Some(Rc::clone(reversed_list.as_ref().unwrap()));

                self.head = original_list;

                let mut first_curr = Some(Rc::clone(self.head.as_ref().unwrap()));
                let mut second_curr = Some(Rc::clone(reversed_list.as_ref().unwrap()));
                let mut palindrome_found = true;

                while second_curr.is_some() && palindrome_found {
                    if second_curr.as_ref().unwrap().borrow().data
                        == first_curr.as_ref().unwrap().borrow().data
                    {
                        let mut next_first: NodeOption = None;
                        let mut next_second: NodeOption = None;

                        match second_curr {
                            Some(ref curr_second) => {
                                if curr_second.as_ref().borrow().next.is_some() {
                                    next_second = Some(Rc::clone(
                                        curr_second.as_ref().borrow().next.as_ref().unwrap(),
                                    ));

                                    match first_curr {
                                        Some(ref curr_first) => {
                                            next_first = Some(Rc::clone(
                                                curr_first.as_ref().borrow().next.as_ref().unwrap(),
                                            ))
                                            // }
                                        }
                                        None => {}
                                    }
                                }
                            }
                            None => {}
                        }

                        first_curr = next_first;
                        second_curr = next_second;
                    } else {
                        palindrome_found = false;
                    }
                }

                is_palindrome = palindrome_found;
            } else {
                if slow_ptr.as_ref().unwrap().borrow().data
                    == self.head.as_ref().unwrap().borrow().data
                {
                    is_palindrome = true;
                }
            }
        }

        return is_palindrome;
    }
}

#[cfg(test)]
mod linked_palindrom_tests {
    use super::*;

    #[test]
    fn is_palindrome_first() {
        let mut list = LinkedList::new_empty();
        list.append_end("M".to_string());
        list.append_end("A".to_string());
        list.append_end("D".to_string());
        list.append_end("A".to_string());
        list.append_end("M".to_string());
        assert_eq!(list.is_linked_palindrome(), true);
    }

    #[test]
    fn is_palindrome_second() {
        let mut list = LinkedList::new_empty();
        list.append_end("node_1".to_string());
        list.append_end("node_2".to_string());
        list.append_end("node_3".to_string());
        list.append_end("node_4".to_string());
        list.append_end("node_5".to_string());
        list.append_end("node_6".to_string());
        list.append_end("node_7".to_string());
        list.append_end("node_8".to_string());

        assert_eq!(list.is_linked_palindrome(), false);
    }

    #[test]
    fn is_palindrome_third() {
        let mut list = LinkedList::new_empty();
        list.append_end("1".to_string());
        list.append_end("2".to_string());
        list.append_end("3".to_string());
        list.append_end("3".to_string());
        list.append_end("2".to_string());
        list.append_end("1".to_string());

        assert_eq!(list.is_linked_palindrome(), true);
    }

    #[test]
    fn is_palindrome_fourth() {
        let mut list = LinkedList::new_empty();
        list.append_end("M".to_string());
        list.append_end("A".to_string());
        list.append_end("D".to_string());
        list.append_end("A".to_string());
        list.append_end("M".to_string());
        list.append_end("E".to_string());

        assert_eq!(list.is_linked_palindrome(), false);
    }

    #[test]
    fn is_palindrome_fifth() {
        let mut list = LinkedList::new_empty();
        assert_eq!(list.is_linked_palindrome(), false);
    }

    #[test]
    fn is_palindrome_sixth() {
        let mut list = LinkedList::new_empty();
        list.append_end("M".to_string());

        assert_eq!(list.is_linked_palindrome(), false);
    }

    #[test]
    fn is_palindrome_seventh() {
        let mut list = LinkedList::new_empty();
        list.append_end("M".to_string());
        list.append_end("M".to_string());

        assert_eq!(list.is_linked_palindrome(), true);
    }

    #[test]
    fn is_palindrome_eight() {
        let mut list = LinkedList::new_empty();
        list.append_end("M".to_string());
        list.append_end("A".to_string());

        assert_eq!(list.is_linked_palindrome(), false);
    }
}
