// BIG THANKS TO ELVIS, I USED HIS ARTICLE TO IMPLEMENT THIS: https://medium.com/@elvis26112009/implementing-linked-lists-in-rust-the-safe-way-8abfb9607c11
//

use super::node::{ListNodeIterator, Node, NodeOption};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
pub struct LinkedList {
    pub head: NodeOption,
    pub tail: NodeOption,
    pub length: usize,
}

impl LinkedList {
    pub fn new_empty() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn new(text: String) -> Self {
        let new_head = Node::new(text);

        LinkedList {
            head: Some(new_head),
            tail: None,
            length: 1,
        }
    }

    pub fn append_start(&mut self, text: String) {
        let new_head = Node::new(text);

        match self.head.take() {
            Some(old_head) => {
                new_head.borrow_mut().next = Some(Rc::clone(&old_head));

                match &self.tail {
                    None => self.tail = Some(Rc::clone(&old_head)),
                    _ => (),
                }
            }
            _ => (),
        }

        self.head = Some(new_head);
        self.length = self.length + 1;
    }

    pub fn append_end(&mut self, text: String) {
        match &self.head {
            Some(head) => {
                let new_tail = Node::new(text);

                match self.tail.take() {
                    Some(old_tail) => old_tail.borrow_mut().next = Some(Rc::clone(&new_tail)),
                    _ => {
                        head.borrow_mut().next = Some(Rc::clone(&new_tail));
                    }
                }

                self.tail = Some(new_tail);
                self.length += 1;
            }
            _ => self.append_start(text),
        }
    }

    pub fn iter_node(&self) -> ListNodeIterator {
        match &self.head {
            Some(head) => ListNodeIterator::new(Some(Rc::clone(head))),
            _ => ListNodeIterator::new(None),
        }
    }

    pub fn print_items(&self) {
        for node in self.iter_node() {
            println!("the data is {}", node.borrow().data);
        }
    }

    pub fn pop_head(&mut self) -> Option<String> {
        self.head.take().map(|old_head| {
            println!("STILL HERE MY PEOPLE");
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    self.head = Some(Rc::clone(&new_head));
                }
                _ => match &self.head {
                    Some(_x) => {}
                    None => self.tail = None,
                },
            }

            self.length -= 1;
            old_head.borrow().data.clone()
        })
    }

    pub fn pop_end(&mut self) -> Option<String> {
        self.tail.take().map(|old_tail| {
            let mut iterator = self.iter_node();
            let mut temp = iterator.next();

            for _ in 0..self.length - 2 {
                temp = iterator.next();
            }

            match temp {
                Some(node) => {
                    node.borrow_mut().next = None;
                    if self.length > 2 {
                        self.tail = Some(Rc::clone(&node));
                    }
                }
                _ => {}
            }

            self.length -= 1;
            old_tail.borrow().data.clone()
        })
    }
}

mod test_linked_rcs_mod {
    #[test]
    fn test_new_empty_list() {
        use super::LinkedList;
        let list = LinkedList::new_empty();

        assert_eq!(
            list,
            LinkedList {
                head: None,
                tail: None,
                length: 0
            }
        )
    }

    #[test]
    fn test_new_list() {
        use super::{LinkedList, Node};

        let list = LinkedList::new("node_1".to_string());

        assert_eq!(
            list,
            LinkedList {
                head: Some(Node::new("node_1".to_string())),
                tail: None,
                length: 1
            }
        )
    }

    #[test]
    fn test_append_list() {
        use super::LinkedList;

        // let mut list = LinkedList::new("node_1".to_string());
        let mut list = LinkedList::new_empty();

        list.append_end("node_2".to_string());
        list.append_end("node_3".to_string());
        list.append_end("node_4".to_string());
        list.pop_head();
        list.pop_head();

        // println!("WHAT THE LIST LOOKS LIKE {:#?}", list);
        list.pop_head();
        // println!("WHAT THE LIST LOOKS LIKE::::::::::::: {:#?}", list);
        list.pop_head();
        // println!("WHAT THE LIST LOOKS LIKE::::::::::::: {:#?}", list);

        // assert_eq!(3, 4);
    }
}
