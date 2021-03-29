#[derive(PartialEq, Eq, Clone, Debug, Default)]
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

    if list.is_empty() {
        return Some(Box::new(ListNode::new(3)));
    }

    for index in (0..=list.len() - 1).rev() {
        let mut node = ListNode::new(list[index]);
        node.next = head;
        head = Some(Box::new(node));
    }

    return head;
}

#[derive(Default)]
pub struct RLL {
    list: Option<Box<ListNode>>,
}

impl RLL {
    pub fn new() -> RLL {
        RLL { list: None }
    }

    pub fn reverse_linked_list(&mut self, head: Option<Box<ListNode>>) {
        let mut node = head.clone();
        match head {
            Some(head_cont) => {
                let mut the_node = std::mem::replace(&mut node, None);

                match &mut the_node {
                    Some(in_node) => {
                        let prior = self.list.clone();
                        in_node.next = prior;
                    }
                    None => {}
                }
                let _ = std::mem::replace(&mut node, the_node);
                self.list = node;
                self.reverse_linked_list(head_cont.next);
            }
            None => {
                return;
            }
        }
    }
}

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut list = RLL::new();
    list.reverse_linked_list(head);
    return list.list;
}

#[cfg(test)]
mod test_reverse_list_cont {

    use super::*;

    #[test]
    fn test_reverse_list() {
        let list = vec![1, 2, 3, 4];

        let ll = make_ll(list);
        let expected_list = vec![4, 3, 2, 1];
        let expected_ll = make_ll(expected_list);

        assert_eq!(reverse_list(ll), expected_ll);

        let list = vec![];

        let ll = make_ll(list);
        let expected_list = vec![];
        let expected_ll = make_ll(expected_list);

        assert_eq!(reverse_list(ll), expected_ll);

        // assert_eq!(3, 4);

        // assert_eq!(3, 4);
    }
}
