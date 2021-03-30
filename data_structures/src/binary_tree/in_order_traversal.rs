use crate::binary_tree::priority_queue::BinaryHeap;

use std::fmt::{Debug, Display};

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Clone + Debug + Copy + Display,
{
    pub fn recursive_in_order_traversal(&self, start: Option<usize>) -> Vec<T> {
        let mut traversal: Vec<T> = vec![];
        if self.heap.len() > 0 {
            if start.is_none() {
                return traversal;
            }

            let curr_index = start.unwrap();
            let left_index = self.get_left_child(curr_index);

            let next_left = self.recursive_in_order_traversal(left_index);

            for value in next_left {
                traversal.push(value);
            }

            // add curr node
            let curr_node = self.heap[curr_index].node;
            traversal.push(curr_node);

            let right_index = self.get_right_child(curr_index);
            let next_right = self.recursive_in_order_traversal(right_index);

            for value in next_right {
                traversal.push(value);
            }

            // let node_at_left = self.heap[next_left];
            // traversal.push()
        }

        return traversal;
    }
}

#[cfg(test)]
mod test_inorder_traversal {
    use crate::binary_tree::priority_queue::{BinaryHeap, TheNode};

    #[test]
    fn one_element_recursive_inorder_test() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });

        let trav = tree.recursive_in_order_traversal(Some(0));

        assert_eq!(trav.len(), 1);
        assert_eq!(trav, vec!["A"]);
    }

    #[test]
    fn double_element_recursive_inorder_test() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode { node: 1, weight: 5 });
        tree.insert(TheNode { node: 2, weight: 7 });

        let trav = tree.recursive_in_order_traversal(Some(0));
        assert_eq!(trav.len(), 2);
        // remember we are dealing with a balanced min priority queue
        assert_eq!(trav, vec![2, 1]);
    }

    #[test]
    fn recursive_inorder_test() {
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

        let trav = tree.recursive_in_order_traversal(Some(0));

        assert_eq!(trav, vec!["F", "B", "A", "E", "D", "J", "Q"]);
    }
}
