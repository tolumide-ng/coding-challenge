use crate::binary_tree::priority_queue::{BinaryHeap, TheNode};
use std::fmt::{Debug, Display};

/// Level order traversal is also called Breadth-first search

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Display + Debug + Copy + Clone,
{
    fn breadth_first_search(&self) -> Vec<T> {
        let mut traversal = vec![];
        let mut queue = vec![];
        let mut curr_position = 0;

        // while last_position < queue.len() ||  {}
        loop {
            let left_index = self.get_left_child(curr_position);
            let right_index = self.get_right_child(curr_position);

            if left_index.is_some() {
                queue.push(left_index);
            }

            if right_index.is_some() {
                queue.push(right_index);
            }

            println!("WHAT THE TRAVERSAL LOOKS LIKE {:#?}", traversal);
            // let right_child = self.get_right_child()
            println!("LENGTH OF THE QUEUE {:#?}", queue.len());
            println!("THE QUEUE ITSELF {:#?}", queue);
            if curr_position > queue.len() {
                break;
            }
            traversal.push(self.heap[curr_position].node);
            curr_position += 1;
        }

        traversal
    }
}

// #[cfg(test)]
mod breadth_first_search {
    use super::*;

    #[test]
    fn test_breadth_first_search() {
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
            tree.breadth_first_search(),
            vec!["E", "B", "J", "F", "A", "D", "Q",]
        );
    }

    #[test]
    fn single_elem_beadth_first_test() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });

        assert_eq!(tree.breadth_first_search(), vec!["A",]);
    }

    #[test]
    fn double_elem_breath_first_test() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });
        tree.insert(TheNode {
            node: "B",
            weight: 2,
        });

        assert_eq!(tree.breadth_first_search(), vec!["B", "A"]);
    }
}
