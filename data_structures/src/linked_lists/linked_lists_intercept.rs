/// Find the point of intersection of two linked lists
/// Space complexity: O(m)
/// Time complexity O(mn)
///
use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

struct LinkedList {
    first_head: Option<Rc<Node>>,
    second_head: Option<Rc<Node>>,
}

impl LinkedList {
    fn get_interception(&self) -> Option<Rc<Node>> {
        let mut intercept: Option<Rc<Node>> = None;

        let mut vec_safe = vec![];
        let mut is_intercept = false;

        match self.first_head {
            Some(ref head) => {
                let mut curr = head;
                vec_safe.push(Rc::clone(curr));

                while head.next.is_some() {
                    curr = head.next.as_ref().unwrap();
                    vec_safe.push(Rc::clone(curr));
                }
            }
            None => return intercept,
        }

        match self.second_head {
            Some(ref head) => {
                let curr = head;
                while head.next.is_some() && !is_intercept {
                    vec_safe.iter().map(|val| {
                        if Rc::as_ptr(curr) == Rc::as_ptr(val) {
                            is_intercept = true;
                            intercept = Some(Rc::clone(curr));
                        }
                    });
                }
                return intercept;
            }
            None => return intercept,
        }
    }
}
