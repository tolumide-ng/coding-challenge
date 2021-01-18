// you can't have a linked cycle in the rust implenetation without the rust compiler shouting in the first place
// but for the sake of writing this down
use std::rc::Rc;

#[derive(Default)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

#[derive(Default)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn has_cycle(&self) -> bool {
        match self.head {
            Some(ref head) => {
                let ref fast = head.next;

                match fast {
                    Some(fast) => {
                        let mut the_slow = head;
                        let mut the_fast = fast;

                        while the_fast.val != the_slow.val {
                            match the_fast.next {
                                Some(ref new_fast) => {
                                    if new_fast.next.is_some() {
                                        the_fast = new_fast.next.as_ref().unwrap();
                                        the_slow = the_slow.next.as_ref().unwrap();
                                    } else {
                                        return false;
                                    }
                                }
                                None => return false,
                            }
                        }
                        return true;
                    }
                    None => {
                        return false;
                    }
                }
            }
            None => return false,
        }
    }
}
