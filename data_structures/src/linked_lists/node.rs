use std::cell::RefCell;
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<Node>>;
pub type NodeOption = Option<NodeRef>;

#[derive(PartialEq, Debug)]
pub struct Node {
    pub data: String,
    pub next: NodeOption,
}

impl Node {
    pub fn new(text: String) -> NodeRef {
        Rc::new(RefCell::new(Node {
            data: text,
            next: None,
        }))
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Node with this data -> '{}' just dropped", self.data);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let node = Node::new("node_1".to_string());

        assert_eq!(
            node,
            Rc::new(RefCell::new(Node {
                data: "node_1".to_string(),
                next: None
            }))
        )
    }
}

pub struct ListNodeIterator {
    current: NodeOption,
}

impl ListNodeIterator {
    pub fn new(start_at: NodeOption) -> Self {
        ListNodeIterator { current: start_at }
    }
}

impl Iterator for ListNodeIterator {
    type Item = NodeRef;

    fn next(&mut self) -> NodeOption {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                result = Some(Rc::clone(current));
                match &current.borrow().next {
                    Some(next_node) => Some(Rc::clone(next_node)),
                    _ => None,
                }
            }
            _ => None,
        };

        result
    }
}
