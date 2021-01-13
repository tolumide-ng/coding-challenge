use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<RefCell<Rc<Node>>>,
}

#[derive(Debug, Clone)]
struct LL {
    head: Option<Rc<Node>>,
    tail: Option<Rc<Node>>,
    total: usize,
}

impl LL {
    fn new() -> Self {
        return LL {
            head: None,
            tail: None,
            total: 0,
        };
    }

    fn add_at_head(&mut self, value: i32) {
        if self.total == 0 {
            let newNode = Rc::new(Node { value, next: None });
            self.head = Some(Rc::clone(&newNode));
            self.tail = Some(Rc::clone(&newNode));
        } else {
            let old_head = mem::take(&mut self.head).unwrap();

            let newNode = Rc::new(Node {
                value,
                next: Some(RefCell::new(Rc::clone(&old_head))),
            });

            self.head = Some(Rc::clone(&newNode));
        }
    }

    fn add_to_tail(&mut self, value: i32) {
        let newNode = Rc::new(Node { value, next: None });
        // if self.total == 0 {
        //     self.head = Some(Rc::clone(&newNode));
        //     self.tail = Some(Rc::clone(&newNode));
        // } else {
        // let old_list = mem::take(&mut self.head).unwrap();

        // let mut curr_node = Rc::clone(&old_list);

        // for value in 0..self.total {
        //     if curr_node.next.is_some() {
        //         curr_node = Rc::clone(curr_node.next.unwrap());
        //     }
        // }
        match &mut self.head {
            Some(head) => {
                let mut curr = head;
                loop {
                    if curr.next.is_some() {
                        curr.next = Some(RefCell::new(Rc::clone(&newNode)))
                    }
                }
            }
            None => {
                self.head = Some(Rc::clone(&newNode));
                self.tail = Some(Rc::clone(&newNode));
            }
        }
        // }
    }

    // fn get(&self, index: i32) -> i32 {
    //     if index < 0 || index > self.total as i32 {
    //         return -1;
    //     }

    //     if index == 0 {
    //         return self.head.unwrap().value;
    //     }

    //     let curr_head = Rc::clone(&self.head.unwrap());
    //     let mut curr_position = curr_head.next;
    //     let mut return_value = -1;

    //     for value in 0..self.total {
    //         match curr_position {
    //             Some(position) => return_value = position.borrow().value,
    //             None => {}
    //         }
    //     }

    //     return return_value;
    // }
}

#[test]
fn singly_ll() {
    let mut list = LL::new();
    list.add_at_head(12);
    list.add_at_head(22);

    // assert_eq!(list.get(0), 12);
}
