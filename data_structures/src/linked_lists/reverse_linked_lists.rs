// use std::cell::RefCell;
// use std::rc::Rc;

// type TypeNode = Option<Rc<Node>>;

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     next: TypeNode,
// }

// #[derive(Debug)]
// struct LinkedList {
//     head: TypeNode,
// }

// impl LinkedList {
//     fn reverse_linked_list(&mut self) {
//         let next_node: TypeNode = None;
//         let mut the_list = self.head.as_ref();

//         loop {
//             match the_list {
//                 Some(ref curr_head) => match curr_head.next {
//                     Some(ref next_head) => {
//                         let next_neighbour = next_head.as_ref().next;
//                         curr_head.next = next_neighbour;
//                         next_head.next = Some(**curr_head);
//                         self.head = Some(*next_head);
//                         match next_neighbour {
//                             Some(ref neighbour) => {
//                                 curr_head.next = Some(*neighbour);

//                                 the_list = Some(neighbour);
//                             }
//                             None => {
//                                 the_list = None;
//                             }
//                         }
//                     }
//                     None => {}
//                 },
//                 None => {
//                     break;
//                 }
//             }
//         }
//     }
// }

// use super::leet_singly_ll;
use crate::linked_lists::leet_singly_ll;
use std::mem;

impl leet_singly_ll::MyLinkedList {
    fn reverse_linked_list(&mut self) {
        // let mut list = self.head;

        let mut curr = match self.head {
            Some(ref mut a) => a,
            None => return,
        };

        while let Some(ref mut new_head) = curr.next {
            // let the_new_head = new_head;
            match new_head.next {
                Some(ref new_neighbour) => {
                    new_head.next = mem::take(&mut new_head.next);
                    // new_head.next = mem::take(new_head);
                }
                None => {
                    // new_head.next = mem::take(dest: &mut T)
                    // new_head.next = self.head;
                }
            }
        }
    }
}
