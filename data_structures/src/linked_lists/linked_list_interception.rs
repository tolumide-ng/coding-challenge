/// Space Complexity O(1)
/// Time Complexity O(mn)
///
use std::rc::Rc;

type TypeNode = Option<Rc<Node>>;

struct Node {
    value: i32,
    next: TypeNode,
}

struct LinkedList {
    first_head: TypeNode,
    second_head: TypeNode,
}

impl LinkedList {
    fn get_interception(&self) -> TypeNode {
        let mut intercept: TypeNode = None;

        let mut is_intercept_a = false;
        let mut is_intercept_b = false;

        let the_a = self.first_head;
        let the_b = self.second_head;

        let not_found = true;

        while not_found {
            match the_a {
                Some(ref derived_a) => {
                    if derived_a.as_ref().next.is_some() && !is_intercept_a {
                        the_a = derived_a.as_ref().next;
                    } else if derived_a.as_ref().next.is_none() && !is_intercept_a {
                        is_intercept_a = true;
                        the_a = self.second_head;
                    }
                }
                None => is_intercept_a = true,
            }

            match the_b {
                Some(ref derived_b) => {
                    if derived_b.as_ref().next.is_some() && !is_intercept_b {
                        the_b = derived_b.as_ref().next;
                    } else if derived_b.as_ref().next.is_none() && !is_intercept_b {
                        is_intercept_b = true;
                        the_b = self.first_head;
                    }
                }
                None => is_intercept_b = true,
            }

            match the_a {
                Some(ref the_derived_a) => {
                    if the_derived_a.as_ref().next.is_some() && is_intercept_a {
                        the_a = the_derived_a.next;
                    }
                }
                None => {}
            }

            match the_b {
                Some(ref the_derived_b) => {
                    if the_derived_b.as_ref().next.is_some() && is_intercept_b {
                        the_b = the_derived_b.next;
                    }

                    if Rc::as_ptr(&the_a.unwrap()) == Rc::as_ptr(&the_b.unwrap()) {
                        intercept = the_a;
                        not_found = false;
                    }
                }
                None => {}
            }
        }

        return intercept;
    }
}
