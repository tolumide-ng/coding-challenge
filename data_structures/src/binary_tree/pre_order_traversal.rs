use crate::binary_tree::priority_queue::{BinaryHeap, Node, TheNode};

use std::fmt::{Debug, Display};

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Display + Debug + Copy + Clone,
{
    fn pre_order_traversal(&self) -> Vec<T> {
        let mut traversal: Vec<T> = vec![];
        let mut stack: Vec<usize> = vec![];

        if self.heap.len() > 0 {
            let mut curr: Option<usize> = Some(0);

            while stack.len() > 0 || curr.is_some() {
                while curr.is_some() {
                    let curr_index = curr.unwrap();
                    traversal.push(self.heap[curr_index].node);

                    let right_child = self.get_right_child(curr_index);

                    if right_child.is_some() {
                        stack.push(right_child.unwrap());
                    }

                    curr = self.get_left_child(curr_index);
                }

                if stack.len() > 0 {
                    curr = stack.pop();
                }
            }
        }

        traversal
    }
}

mod test_preorder_traversal {
    use super::*;

    #[test]
    fn iterative_preorder_test() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });
        tree.insert(TheNode {
            node: "B",
            weight: 2,
        });
        tree.insert(TheNode {
            node: "J",
            weight: 3,
        });
        tree.insert(TheNode {
            node: "F",
            weight: 100,
        });
        tree.insert(TheNode {
            node: "E",
            weight: 1,
        });
        tree.insert(TheNode {
            node: "D",
            weight: 10,
        });
        tree.insert(TheNode {
            node: "Q",
            weight: 4,
        });

        assert_eq!(
            tree.pre_order_traversal(),
            vec!["E", "B", "F", "A", "J", "D", "Q",]
        );
    }
}
