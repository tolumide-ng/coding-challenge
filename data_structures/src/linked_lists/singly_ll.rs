use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Clone, Debug)]
struct LinkedNode {
    value: i32,
    next: Option<Box<RefCell<LinkedNode>>>,
}

#[derive(Clone, Debug)]
struct MyLinkedList {
    head: Option<LinkedNode>,
    tail: Option<LinkedNode>,
    total: usize,
}

impl MyLinkedList {
    fn new() -> Self {
        return MyLinkedList {
            head: None,
            tail: None,
            total: 0,
        };
    }

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index > *&self.total as i32 {
            return -1;
        }

        let mut curr_val: LinkedNode = self.head.clone().unwrap();

        for _ in 0..=index as usize {
            match curr_val.next {
                // Some(the_val) => curr_val = the_val.into_inner().as_ref().clone(),
                Some(the_val) => curr_val = the_val.as_ref().clone().borrow(),

                None => {}
            }
        }

        return curr_val.value as i32;
    }

    fn add_at_head(&mut self, val: i32) {
        let new_next = mem::take(&mut self.head);
        // new_node.next::Rc

        match new_next {
            Some(the_list) => {
                let new_node = LinkedNode {
                    value: val,
                    next: Some(Box::new(the_list)),
                };
                self.head = Some(new_node);
            }

            None => {
                self.head = Some(LinkedNode {
                    value: val,
                    next: None,
                });

                self.tail = self.head.clone();
            }
        }
    }

    fn add_to_tail(&mut self, val: i32) {
        match &self.head {
            Some(head) => {
                // let mut new_head = mem::take(&mut self.head).unwrap();
                let mut new_head = self.head.clone().unwrap();

                for index in 0..self.total {
                    match new_head.next {
                        Some(the_next) => new_head = *the_next,
                        None => {}
                    }
                }
            }
            None => {
                self.head = Some(LinkedNode {
                    value: val,
                    next: None,
                });

                self.tail = self.head.clone();
            }
        }
        // let curr_tail = mem::take(&mut self.head);

        // match curr_tail {
        //     Some(mut value) => {
        //         value.next = Some(Rc::new(LinkedNode {
        //             value: val,
        //             next: None,
        //         }));
        //     }
        //     None => {
        //         // can't add an item to tail if there was nothing there in the first place
        //     }
        // }
    }

    // fn add_at_index(&mut self, val: i32) {
    //     let mut curr_val: LinkedNode = self.head.clone().unwrap();
    //     let we_move: Option<LinkedNode> = None;

    //     for value in 0..val {
    //         if value == val {
    //             match curr_val.next {
    //                 // Some(the_val) => curr_val = mem::take()
    //             }
    //         }
    //     }
    // }
}

#[test]
fn linked_ll_initialization() {
    let mut list = MyLinkedList::new();

    assert!(list.head.is_none());
    assert!(list.head.is_none());
    assert!(list.head.is_none());

    assert!(list.tail.is_none());

    list.add_at_head(5);

    assert_eq!(list.get(0), 5);

    assert!(list.head.is_some());
    assert_eq!(list.head.unwrap().value, 5);
}
