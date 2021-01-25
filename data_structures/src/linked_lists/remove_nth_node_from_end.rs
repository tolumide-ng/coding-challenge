/// Linked List traversal and removal of nth element from the end.
/// Solution:
/// Our solution should be at (L-n+1), where L is the length og the linkedList(LL) and n is the node from the end of the linked list as provided
/// Create a dummy node and attach the LL as the next of the dummy node, this would help incase we had only one node and were remove the 0th item from the end
/// Get the length of the LL by traversing through it and save it as L, now get the value of (L-n+1) which should now be (L-n) owing to the addition of the dummy node
/// Save this value as d, traverse the LL again, and remove the element at the dth position
///
use std::rc::Rc;

struct Node {
    val: i32,
    next: Option<Rc<Node>>,
}

struct LinkedList {
    head: Option<Rc<Node>>,
}

impl LinkedList {
    fn remove_nth_from_end(&mut self, index_from_end: usize) -> Option<Rc<Node>> {
        let mut list_length = 0;

        let dummy_head = Rc::new(Node {
            val: 3,
            next: self.head.clone(),
        });
        while dummy_head.next.is_some() {
            list_length += 1;
        }

        let exact_index = list_length - index_from_end;

        let curr_node = dummy_head;

        for index in 1..=exact_index {
            if index != exact_index {
                curr_node = curr_node.next.unwrap();
            } else {
                let mut left_hand = curr_node.next;
                let mut right_hand: Option<Rc<Node>> = None;
                if left_hand.unwrap().next.is_some() {
                    left_hand.unwrap().next = None;
                    if left_hand.unwrap().next.unwrap().next.is_some() {
                        right_hand = left_hand.unwrap().next.unwrap().next;
                    }
                    left_hand.unwrap().next = right_hand;
                }
            }
        }

        return dummy_head.next;
    }
}
