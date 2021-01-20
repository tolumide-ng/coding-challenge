// find the node beginning of a lopp in a linked cycle

use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

struct LinkedList {
    head: Option<Rc<Node>>,
}

impl LinkedList {
    fn cycle_position(&self) -> i32 {
        let mut loop_start = -1;
        match self.head {
            Some(ref head) => {
                let mut slow = head;
                let mut cycle_found = false;
                match head.next {
                    Some(ref fast) => {
                        let mut fast = fast;
                        loop {
                            if fast.value != slow.value {
                                if fast.next.as_ref().is_some() {
                                    fast = fast.next.as_ref().unwrap();
                                    slow = slow.next.as_ref().unwrap();
                                    if fast.next.as_ref().is_some() && !cycle_found {
                                        fast = fast.next.as_ref().unwrap();
                                    } else {
                                        break;
                                    }
                                } else {
                                    break;
                                }
                            } else {
                                if cycle_found && fast.value == slow.value {
                                    loop_start = fast.value;
                                    break;
                                } else {
                                    cycle_found = true;
                                    slow = head;
                                }
                            }
                        }
                    }
                    None => return loop_start,
                }

                return loop_start;
            }
            None => return loop_start,
        }
    }
}
