pub type Node = Option<Box<ListNode>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Node,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn make_ll(list: Vec<i32>) -> Node {
    let mut head: Node = None;

    for index in (0..=list.len() - 1).rev() {
        let mut node = ListNode::new(list[index]);
        node.next = head;
        head = Some(Box::new(node));
    }

    // println!("WHAT THE HEAD LOOKS LIKE {:#?}", head);

    return head;
}
