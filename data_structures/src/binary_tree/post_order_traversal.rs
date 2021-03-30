use crate::binary_tree::priority_queue::BinaryHeap;

use std::fmt::{Debug, Display};

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Clone + Debug + Copy + Display,
{
    pub fn recursive_post_order_traversal(&self, start: Option<usize>) -> Vec<T> {
        let mut traversal: Vec<T> = vec![];

        if self.heap.len() > 0 {
            if start.is_none() {
                return traversal;
            }

            let curr_index = start.unwrap();
            let left_index = self.get_left_child(curr_index);

            let next_left = self.recursive_post_order_traversal(left_index);

            for value in next_left {
                traversal.push(value);
            }

            // right node now
            let right_index = self.get_right_child(curr_index);
            let next_right = self.recursive_post_order_traversal(right_index);

            for value in next_right {
                traversal.push(value);
            }

            // add the curr value
            let curr_node = self.heap[curr_index].node;
            traversal.push(curr_node);
        }

        traversal
    }
}

#[cfg(test)]
mod test_post_order_traversal {
    use crate::binary_tree::priority_queue::{BinaryHeap, TheNode};

    #[test]
    fn test_post_order_traversal() {
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

        let trav = tree.recursive_post_order_traversal(Some(0));

        println!("THE REAL TREEE >>>>>>>>> {:#?}", tree);

        assert_eq!(trav, vec!["F", "A", "B", "D", "Q", "J", "E"]);
    }
}
