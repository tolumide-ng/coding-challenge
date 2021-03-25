use data_structures::linked_lists::linked_rcs;

// definition for singly-linked list
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn make_ll(list: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head: Option<Box<ListNode>> = None;

    for index in (0..=list.len() - 1).rev() {
        let mut node = ListNode::new(list[index]);
        node.next = head;
        head = Some(Box::new(node));
    }

    // println!("WHAT THE HEAD LOOKS LIKE {:#?}", head);

    return head;
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(list) => {
            let mut first = list;
            let mut second = first.clone().next;

            let mut the_pair: Option<Box<ListNode>> = None;

            if second.is_some() {
                let next = second.as_ref().unwrap().next.as_ref();

                match next {
                    Some(next_list) => {
                        the_pair = swap_pairs(Some(next_list.clone()));
                    }
                    None => {}
                }
            }

            first.next = the_pair;
            if second.is_some() {
                let mut the_second = std::mem::replace(&mut second, None);

                match &mut the_second {
                    Some(the_list) => {
                        the_list.next = Some(first);
                    }
                    None => {}
                }

                std::mem::replace(&mut second, the_second);

                return second;
            } else {
                return Some(first);
            }
        }
        None => return None,
    }
}

pub fn swap_pairs2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.and_then(|mut n| match n.next {
        Some(mut m) => {
            n.next = swap_pairs(m.next);
            m.next = Some(n);
            Some(m)
        }
        None => Some(n),
    })
}

#[cfg(test)]
mod test_swap_pairs_cont {
    use super::*;

    #[test]
    fn test_function_can_make_linked_list() {
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let ll = make_ll(list);

        assert!(ll.is_some());

        assert_eq!(ll.clone().unwrap().val, 1);

        assert_eq!(ll.clone().unwrap().next.unwrap().val, 2);
    }

    #[test]
    fn test_swap_pairs() {
        let list = vec![1, 2, 3, 4];

        let ll = make_ll(list);
        let expected_list = vec![2, 1, 4, 3];
        let expected_ll = make_ll(expected_list);

        assert_eq!(swap_pairs(ll), expected_ll);

        let list = vec![1, 2, 3, 4, 5, 6, 7];

        let ll = make_ll(list);
        let expected_list = vec![2, 1, 4, 3, 6, 5, 7];
        let expected_ll = make_ll(expected_list);

        assert_eq!(swap_pairs(ll), expected_ll);
    }
}
