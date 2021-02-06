/// Depending on how you index the priority_qeue, the right and left child index would be influenced

#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    pub weight: i8,
    pub node: T,
    time_added: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct TheNode<T> {
    pub weight: i8,
    pub node: T,
}

impl<T> Node<T>
where
    T: Debug + Clone + Copy,
{
    pub fn new(data: T, weight: i8, time_added: u8) -> Node<T> {
        Node {
            weight,
            node: data,
            time_added,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    pub heap: Vec<Node<T>>,
    max_size: usize,
    pub size: usize,
    realistic_size: usize,
}

use std::fmt::{Debug, Display};

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Display + Debug + Copy + Clone,
{
    pub fn new(max_size: usize) -> Self {
        BinaryHeap {
            heap: Vec::with_capacity(max_size),
            max_size,
            size: 0,
            realistic_size: 0,
        }
    }

    pub fn get_left_child(&self, index: usize) -> Option<usize> {
        let left_index = (index * 2) + 1;

        if left_index < self.size {
            return Some(left_index);
        }

        None
    }

    pub fn get_right_child(&self, index: usize) -> Option<usize> {
        let right_index = (index * 2) + 2;

        if right_index < self.size {
            return Some(right_index);
        }
        None
    }

    pub fn get_parent(&self, index: usize) -> Option<usize> {
        if index < self.size {
            if index <= 2 {
                return Some(0);
            }
            if index % 2 == 0 {
                let parent = (index / 2) - 1;
                return Some(parent);
            }

            let parent = (index - 1) / 2;
            return Some(parent);
        }
        None
    }

    pub fn shift_down(&mut self, index: usize) {
        let mut new_index = index;
        let left_child_index = self.get_left_child(index);

        match left_child_index {
            Some(child_index) => {
                if child_index < self.size {
                    let current = self.heap[new_index];
                    let child = self.heap[child_index];

                    if current.weight > child.weight
                        || (current.weight == child.weight && current.time_added > child.time_added)
                    {
                        new_index = child_index;
                    }
                }
            }
            None => {}
        }

        let right_child_index = self.get_right_child(index);

        match right_child_index {
            Some(child_index) => {
                if child_index < self.size {
                    let current = self.heap[new_index];
                    let child = self.heap[child_index];

                    if current.weight > child.weight
                        || (current.weight == child.weight && current.time_added > child.time_added)
                    {
                        new_index = child_index
                    }
                }
            }
            None => {}
        }

        if index != new_index {
            let current = self.heap[index];
            let child = self.heap[new_index];

            let _ = std::mem::replace(&mut self.heap[index], child);
            let _ = std::mem::replace(&mut self.heap[new_index], current);

            self.shift_down(new_index);
        }
    }

    pub fn shift_up(&mut self, index: usize) {
        if index > 0 {
            let parent_index = self.get_parent(index).unwrap();
            let parent = self.heap[parent_index];
            let current = self.heap[index];

            if current.weight < parent.weight
                || (current.weight == parent.weight && current.time_added < parent.time_added)
            {
                let _ = std::mem::replace(&mut self.heap[parent_index], current);
                let _ = std::mem::replace(&mut self.heap[index], parent);

                self.shift_up(parent_index);
            }
        }
    }

    pub fn insert(&mut self, new_node: TheNode<T>) {
        if self.size != self.max_size {
            self.size += 1;

            self.heap.push(Node {
                node: new_node.node,
                time_added: self.size as u8,
                weight: new_node.weight,
            });

            self.shift_up(self.size - 1);
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        if self.size > 1 {
            let last_value = self.heap.pop().unwrap();
            self.size -= 1;
            let result = std::mem::replace(&mut self.heap[0], last_value);
            self.shift_down(0);
            return Some(result.node);
        }

        if self.size == 1 {
            let result = self.heap.pop().unwrap();
            self.size -= 1;
            return Some(result.node);
        }

        return None;
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.size > index {
            let result = self.heap[index];

            let dummy_node = Node::new(result.node, i8::MIN, 0);

            let _ = std::mem::replace(&mut self.heap[index], dummy_node);

            self.shift_up(index);
            self.extract_min();
            return Some(result.node);

            // OR YOU COULD JUST DO:
            // CHANGE THE PRIORITY OF THE ITEM self.change_priority(index, i8:MIN);
            //  ONCE DONE EXTRACT MIN()
        }
        return None;
    }

    pub fn replace_element(&mut self, index: usize, node: Node<T>) {
        if index < self.size {
            let old_value = std::mem::replace(&mut self.heap[index], node);
            if node.weight > old_value.weight {
                self.shift_down(index);
            }
            if node.weight < old_value.weight {
                self.shift_up(index);
            }
        }
    }

    pub fn change_prority(&mut self, index: usize, weight: usize) {
        if self.size > index {
            let old_value = self.heap[index];
            let _ = std::mem::replace(
                &mut self.heap[index],
                Node {
                    node: old_value.node,
                    weight: weight as i8,
                    time_added: old_value.time_added,
                },
            );
            if weight > old_value.weight as usize {
                self.shift_down(index)
            }

            if weight < old_value.weight as usize {
                self.shift_up(index);
            }
        }
    }

    pub fn total_length(&self) -> usize {
        return self.size;
    }

    pub fn get_min(&self) -> Option<T> {
        if self.size > 0 {
            return Some(self.heap[0].node);
        }
        return None;
    }
}

mod test_min_priority {
    use super::*;

    #[test]
    fn min_priority_insertion() {
        let mut tree = BinaryHeap::new(10);

        assert_eq!(tree.total_length(), 0);
        assert!(tree.extract_min().is_none());

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });

        assert_eq!(tree.total_length(), 1);
        assert!(tree.extract_min().is_some());
    }

    #[test]
    fn min_priority_order() {
        let mut tree = BinaryHeap::new(10);

        assert_eq!(tree.total_length(), 0);
        assert!(tree.extract_min().is_none());
        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });
        tree.insert(TheNode {
            node: "B",
            weight: 3,
        });
        tree.insert(TheNode {
            node: "C",
            weight: 1,
        });
        tree.insert(TheNode {
            node: "D",
            weight: 100,
        });
        tree.insert(TheNode {
            node: "E",
            weight: 5,
        });

        assert_eq!(tree.total_length(), 5);
        let curr_min = tree.extract_min();
        assert!(curr_min.is_some());
        assert_eq!(curr_min.unwrap(), "C");

        let curr_min = tree.extract_min();
        assert!(curr_min.is_some());
        assert_eq!(curr_min.unwrap(), "B");
    }

    #[test]
    fn order_after_removal() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });
        tree.insert(TheNode {
            node: "B",
            weight: 3,
        });
        tree.insert(TheNode {
            node: "C",
            weight: 1,
        });
        tree.insert(TheNode {
            node: "D",
            weight: 100,
        });
        tree.insert(TheNode {
            node: "E",
            weight: 5,
        });

        let removed_element = tree.remove(1);
        assert!(removed_element.is_some());
        assert_eq!(tree.extract_min().unwrap(), "C");
        tree.insert(TheNode {
            node: "E",
            weight: 0,
        });
        let min = tree.extract_min();

        assert_eq!(min.unwrap(), "E");
    }

    #[test]
    fn test_change_priority() {
        let mut tree = BinaryHeap::new(10);

        tree.insert(TheNode {
            node: "A",
            weight: 5,
        });
        tree.insert(TheNode {
            node: "B",
            weight: 3,
        });
        tree.insert(TheNode {
            node: "C",
            weight: 1,
        });
        tree.insert(TheNode {
            node: "D",
            weight: 100,
        });
        tree.insert(TheNode {
            node: "E",
            weight: 5,
        });

        assert_eq!(tree.get_min().unwrap(), "C");

        tree.change_prority(0, 3);

        assert_ne!(tree.get_min().unwrap(), "C");

        assert_eq!(tree.get_min().unwrap(), "B");
    }
}
