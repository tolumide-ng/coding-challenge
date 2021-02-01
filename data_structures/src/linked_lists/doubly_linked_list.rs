use super::dnode::{DNode, DNodeOption, ListDNodeIterator};
use std::cell::RefCell;
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Debug)]
pub struct DLinkedList<T: Debug + Display> {
    pub head: DNodeOption<T>,
    pub tail: DNodeOption<T>,
    length: usize,
}

impl<T> DLinkedList<T>
where
    T: Debug + Display,
{
    // pub fn new_empty() -> Self {
    //     DLinkedList {
    //         head: None,
    //         tail: None,
    //         length: 0,
    //     }
    // }

    pub fn new(data: T) -> Self {
        let new_node = DNode::new(data);

        DLinkedList {
            head: Some(Rc::clone(&new_node)),
            tail: Some(Rc::clone(&new_node)),
            length: 1,
        }
    }

    pub fn append_start(&mut self, data: T) {
        let new_head = DNode::new(data);

        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));
                old_head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
            }
            _ => {}
        }

        self.head = Some(new_head);
        self.length += 1;
    }

    pub fn append_end(&mut self, data: T) {
        match &self.head {
            Some(head) => {
                let new_tail = DNode::new(data);

                match self.tail.take() {
                    Some(old_tail) => {
                        old_tail.borrow_mut().next = Some(Rc::clone(&new_tail));
                        new_tail.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                    }
                    _ => {
                        // head.borrow_mut().next = Some(Rc::clone(&new_tail));
                        // new_tail.borrow().next = Some(Rc::downgrade(&head));
                    }
                }

                self.tail = Some(new_tail);
            }
            _ => self.append_start(data),
        }
        self.length += 1;
    }

    pub fn iter_node(&self) -> ListDNodeIterator<T> {
        match &self.head {
            Some(head) => ListDNodeIterator::new(Some(Rc::clone(head))),
            _ => ListDNodeIterator::new(None),
        }
    }

    pub fn print_items(&self) {
        for node in self.iter_node() {
            println!("the data is {}", node.borrow().data);
        }
    }

    pub fn pop_head(&mut self) -> DNodeOption<T> {
        let old_head = self.head.take();
        let mut new_head: DNodeOption<T> = None;

        match old_head {
            Some(ref the_old_head) => {
                match the_old_head.borrow().next {
                    Some(ref the_new_head) => {
                        the_new_head.borrow_mut().prev = None;
                        new_head = Some(Rc::clone(the_new_head));
                    }
                    None => {
                        self.tail = None;
                    }
                }
                the_old_head.borrow_mut().next = None;
                self.length -= 1;
            }
            None => {}
        }

        self.head = new_head;

        return old_head;
    }

    pub fn pop_end(&mut self) -> DNodeOption<T> {
        let mut the_old_tail: DNodeOption<T> = None;

        self.tail.take().map(|old_tail| {
            the_old_tail = Some(Rc::clone(&old_tail));

            let mut iterator = self.iter_node();
            let mut temp = iterator.next();
            // let mut temp: DNodeOption<T> = Some(Rc::clone(&iterator.current.as_ref().unwrap()));

            if self.length > 1 {
                for _ in 0..self.length - 2 {
                    temp = iterator.next();
                }
            }

            match temp {
                Some(node) => {
                    if node.borrow().next.is_some() {
                        node.borrow_mut().next = None;
                        self.tail = Some(Rc::clone(&node));
                    } else {
                        self.head = None;
                        self.tail = None;
                    }
                }
                _ => {}
            }
            self.length -= 1;
        });

        return the_old_tail;
    }

    pub fn get_nth_elem(&self, index: i8) -> DNodeOption<T> {
        let mut curr_elem: DNodeOption<T> = None;

        // this if case has taken care of the edge cases
        if index >= self.length as i8 || index < 0 {
            return curr_elem;
        }

        match &self.head {
            Some(the_head) => {
                curr_elem = Some(Rc::clone(the_head));
                let mut curr_index = 0;

                while curr_elem.is_some() && curr_index < index {
                    curr_index += 1;
                    curr_elem = Some(Rc::clone(
                        curr_elem.unwrap().borrow().next.as_ref().unwrap(),
                    ));
                }
            }
            None => {}
        }

        return curr_elem;
    }

    pub fn add_element_to_nth_position(&mut self, data: T, index: usize) {
        // let the_elem = DNode::new(data);

        if index > self.length {
            return;
        }

        if index == 0 {
            self.append_start(data);
            return;
        }

        if index == self.length {
            self.append_end(data);
            return;
        }

        let mut next_elem = Some(Rc::clone(self.head.as_ref().unwrap()));
        let mut prev_elem: DNodeOption<T> = None;
        let mut curr_index = 0;

        while curr_index < index {
            curr_index += 1;

            prev_elem = Some(Rc::clone(&next_elem.as_ref().unwrap()));

            next_elem = Some(Rc::clone(
                &prev_elem.as_ref().unwrap().borrow().next.as_ref().unwrap(),
            ));
        }

        let the_elem = DNode::new(data);

        the_elem.borrow_mut().prev = Some(Rc::downgrade(&prev_elem.as_ref().unwrap()));
        prev_elem.unwrap().borrow_mut().next = Some(Rc::clone(&the_elem));

        the_elem.borrow_mut().next = Some(Rc::clone(&next_elem.as_ref().unwrap()));
        next_elem.unwrap().borrow_mut().prev = Some(Rc::downgrade(&the_elem));
    }

    pub fn delete_nth_element(&mut self, index: usize) {
        if index >= self.length {
            return;
        }

        if index == 0 {
            self.pop_head();
            return;
        }

        if index == self.length - 1 {
            self.pop_end();
            return;
        }

        let mut curr_elem = Some(Rc::clone(self.head.as_ref().unwrap()));
        let mut curr_index = 1;
        let mut next_elem: DNodeOption<T> = None;

        while curr_index < index {
            println!("within the cubicle");
            curr_index += 1;
            let prev_elem = Some(Rc::clone(&curr_elem.as_ref().unwrap()));

            curr_elem = Some(Rc::clone(
                &prev_elem.as_ref().unwrap().borrow().next.as_ref().unwrap(),
            ));
        }

        if curr_elem.as_ref().unwrap().borrow().next.is_some() {
            next_elem = Some(Rc::clone(
                curr_elem
                    .as_ref()
                    .unwrap()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap()
                    .as_ref()
                    .borrow()
                    .next
                    .as_ref()
                    .unwrap(),
            ));
        }

        curr_elem.as_ref().unwrap().borrow_mut().next =
            Some(Rc::clone(next_elem.as_ref().unwrap()));

        next_elem.as_ref().unwrap().borrow_mut().prev =
            Some(Rc::downgrade(curr_elem.as_ref().unwrap()));
    }
}

mod test_doubly_linked_list {
    use super::*;

    #[test]
    fn test_new_empty_dlist() {
        let mut list = DLinkedList::new("node_1");

        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_1".to_string()
        );

        list.append_start("node_a");

        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_a".to_string()
        );
    }

    #[test]
    fn test_append_to_end() {
        let mut list = DLinkedList::new("node_1");

        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_1".to_string()
        );

        list.append_end("node_2");

        match list.tail {
            Some(ref tail) => {
                assert_eq!(tail.as_ref().borrow().data, "node_2");
                assert!(tail.as_ref().borrow().prev.is_some());
                assert!(tail.as_ref().borrow().next.is_none());
            }
            None => {}
        }
    }

    #[test]
    fn test_pop_head_one_elem() {
        let mut list = DLinkedList::new("node_1");

        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_1".to_string()
        );

        list.pop_head();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
    }

    #[test]
    fn test_pop_head_multi_elem() {
        // Pop from head a doubly-linked list that has multi elements
        let mut list = DLinkedList::new("node_1");
        list.append_end("node_2");
        list.append_end("node_3");

        assert_eq!(list.length, 3);
        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_1".to_string()
        );

        list.pop_head();

        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_2".to_string()
        );

        match list.head {
            Some(ref head) => {
                assert_eq!(head.borrow().data, "node_2".to_string());
                match head.borrow().next {
                    Some(ref next_head) => {
                        assert_eq!(next_head.borrow().data, "node_3".to_string());
                    }
                    None => {}
                }
            }
            None => {}
        }

        match list.tail {
            Some(ref tail) => {
                assert_eq!(tail.borrow().data, "node_3".to_string());
            }
            None => {}
        }
    }

    #[test]
    fn test_pop_end_single_elem() {
        let mut list = DLinkedList::new("node_1");
        list.append_end("node_2");
        list.append_end("node_3");

        match list.tail {
            Some(ref tail) => {
                assert_eq!(tail.borrow().data, "node_3".to_string());
            }
            None => {}
        }

        list.pop_end();

        match list.tail {
            Some(ref tail) => {
                assert_eq!(tail.borrow().data, "node_2".to_string());
            }
            None => {}
        }

        list.pop_end();

        match list.tail {
            Some(ref tail) => {
                assert_eq!(tail.borrow().data, "node_1".to_string());
            }
            None => {}
        }

        list.pop_end();

        assert!(list.tail.is_none());
        assert!(list.head.is_none());
    }

    #[test]
    fn test_get_nth_element() {
        let mut list = DLinkedList::new("node_1");
        list.append_end("node_2");
        list.append_end("node_3");
        list.append_end("node_4");
        list.append_end("node_5");

        let nth_element = list.get_nth_elem(1);

        assert_eq!(nth_element.as_ref().unwrap().borrow().data, "node_2");
        assert_ne!(nth_element.as_ref().unwrap().borrow().data, "node_3");

        let zeroth_element = list.get_nth_elem(0);
        assert_eq!(zeroth_element.as_ref().unwrap().borrow().data, "node_1");
        assert_ne!(zeroth_element.as_ref().unwrap().borrow().data, "nodey_1");

        let fourth_element = list.get_nth_elem(4);
        assert_eq!(fourth_element.as_ref().unwrap().borrow().data, "node_5");
        assert_ne!(fourth_element.as_ref().unwrap().borrow().data, "node_4");

        let fifth_element = list.get_nth_elem(5);
        assert!(fifth_element.as_ref().is_none());

        let non_existent = list.get_nth_elem(-2);
        assert!(non_existent.as_ref().is_none());
    }

    #[test]
    fn insert_elemenet() {
        let mut list = DLinkedList::new("node_1".to_string());
        list.append_end("node_2".to_string());

        assert_eq!(
            list.head.as_ref().unwrap().borrow().data,
            "node_1".to_string()
        );

        list.add_element_to_nth_position("node_3".to_string(), 1);

        match list.head {
            Some(ref head) => match head.borrow().next {
                Some(ref new_head) => {
                    assert_eq!(new_head.borrow().data, "node_3".to_string());

                    match new_head.borrow().next {
                        Some(ref shifted_elem) => {
                            assert_eq!(shifted_elem.borrow().data, "node_2".to_string());
                        }
                        None => {}
                    }
                }
                None => {}
            },
            None => {}
        }

        list.add_element_to_nth_position("node_0".to_string(), 0);
        assert_eq!(list.head.as_ref().unwrap().borrow().data, "node_0");

        list.add_element_to_nth_position("node_20".to_string(), 20);
        assert_eq!(list.head.as_ref().unwrap().borrow().data, "node_0");
        assert_ne!(list.tail.as_ref().unwrap().borrow().data, "node_20");
    }

    #[test]
    fn test_delete_nth_element() {
        let mut list = DLinkedList::new(1);
        list.append_end(2);
        list.append_end(3);
        list.append_end(4);

        // Asserts that the 0th element is 1
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 1);
        // assert_eq!(list.head.as_ref().borrow().data, 1);
        list.delete_nth_element(0);
        // Asserts that the 0th element after the deletion is 2
        assert_eq!(list.head.as_ref().unwrap().borrow().data, 2);
        match list.head {
            Some(ref the_head) => {
                assert_eq!(the_head.as_ref().borrow().data, 2);

                match the_head.as_ref().borrow().next {
                    Some(ref next_element) => {
                        assert_eq!(next_element.as_ref().borrow().data, 3);
                    }
                    None => {}
                }
            }
            None => {}
        }

        list.delete_nth_element(1);
        // Asserts that the 0th element after the deletion is still 2
        match list.head {
            Some(ref the_head) => {
                assert_eq!(the_head.as_ref().borrow().data, 2);

                match the_head.as_ref().borrow().next {
                    Some(ref next_element) => {
                        // Asserts that the first element after the deletion is now 4
                        assert_eq!(next_element.as_ref().borrow().data, 4);
                    }
                    None => {}
                }
            }
            None => {}
        }

        // Assert deletion of the last element on the linked list
        list.delete_nth_element(3);

        match list.tail {
            Some(ref the_tail) => {
                assert_eq!(the_tail.as_ref().borrow().data, 4);
            }
            None => {}
        }

        //  Asserts the list does not crash when a user tries to delete a non existing element
        list.delete_nth_element(8);
        assert_eq!(list.tail.as_ref().unwrap().borrow().data, 4);

        list.delete_nth_element(0);
        list.delete_nth_element(0);

        // Asserts the function does not crash when a user tries to delete an empty list
        list.delete_nth_element(0);
        assert!(list.head.as_ref().is_none());
    }
}
