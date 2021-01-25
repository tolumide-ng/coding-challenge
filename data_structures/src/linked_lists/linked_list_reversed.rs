use std::cell::RefCell;
use std::rc::Rc;

type SpecNode = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    value: i32,
    next: SpecNode,
}

#[derive(Debug)]
struct LinkedList {
    head: SpecNode,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add(&mut self, value: i32) {
        match &mut self.head {
            Some(head) => {
                let mut curr = Rc::clone(head);

                loop {
                    if curr.borrow().next.is_none() {
                        let the_val = Rc::new(RefCell::new(Node { value, next: None }));
                        curr.borrow_mut().next = Some(Rc::clone(&the_val));
                        break;
                    }

                    if curr.borrow().next.is_some() {
                        curr = match curr.clone().borrow().next.clone() {
                            Some(x) => Rc::clone(&x),
                            None => break,
                        }
                    }
                }
            }
            None => self.head = Some(Rc::new(RefCell::new(Node { value, next: None }))),
        }
    }

    fn reverse_list(&mut self) {
        let next_node: SpecNode = None;
        let mut the_list = self.head.clone();

        loop {
            match the_list.clone() {
                Some(ref curr_head) => match &curr_head.borrow().next.clone() {
                    Some(next_head) => {
                        let next_neighbor = next_head.borrow().next.clone();
                        the_list = next_neighbor.clone();
                        curr_head.borrow_mut().next = next_neighbor;
                        println!("FIRST HERE::::::::::::::");
                        next_head.borrow_mut().next = Some(Rc::clone(&curr_head));
                        self.head = Some(Rc::clone(next_head));
                    }
                    None => break,
                },
                None => {
                    break;
                }
            }
        }
    }
}

#[test]
fn wendy_test() {
    let mut list = LinkedList::new();
    list.add(10);
    list.add(40);
    list.add(15);
    println!("THE LIST AFTERALL>>>>>>> {:#?}", list);
    list.reverse_list();
    println!("THE LIST AFTER RVERSAL >>>>>>>>>>> {:#?}", list);
    assert_eq!(4, 3);
}
