use crate::binary_tree::priority_queue::BinaryHeap;

use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ancestor {
    pub found_x: bool,
    pub found_y: bool,
    pub ancestor: Option<usize>,
}

impl Ancestor {
    pub fn get_ancestor<T: PartialEq + Eq + Display + Debug + Copy + Clone>(
        &self,
        x: T,
        y: T,
        root: usize,
    ) -> Option<usize> {
        // let left_child =
        println!("{}{}{}", x, y, root);

        return None;
    }
}

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Display + Debug + Copy + Clone,
{
    pub fn get_common_ancestor(&self, x: T, y: T) -> Option<usize> {
        // Since we're checking for an acestor of 2 items, the size of the heap must be atleast 2
        if self.size <= 1 {
            return None;
        }

        let mut the_ancestor: Ancestor = Default::default();

        let common_ancestor = self.get_ancestor(x, y, Some(0), &mut the_ancestor);

        if the_ancestor.found_x && the_ancestor.found_y {
            return common_ancestor;
        }

        return None;
    }

    pub fn get_ancestor(
        &self,
        x: T,
        y: T,
        root: Option<usize>,
        dic: &mut Ancestor,
    ) -> Option<usize> {
        let mut value: Option<usize> = None;

        match root {
            Some(the_root) => {
                if self.heap[the_root].node == x {
                    dic.found_x = true;
                    value = Some(the_root);
                }

                if self.heap[the_root].node == y {
                    dic.found_y = true;
                    value = Some(the_root);
                }

                let left_child = self.get_left_child(the_root);
                let right_child = self.get_right_child(the_root);

                let first = self.get_ancestor(x, y, left_child, dic);
                let second = self.get_ancestor(x, y, right_child, dic);

                if first.is_some() {
                    if value.is_none() {
                        value = first
                    }
                }

                if second.is_some() {
                    if value.is_none() {
                        value = second
                    }
                }

                if first.is_some() && second.is_some() {
                    value = root;
                    // return root;
                }
            }
            None => {}
        }

        // if dic.found_x && dic.found_y {}
        return value;
    }
}

#[cfg(test)]
mod test_lca {
    use crate::binary_tree::priority_queue::{BinaryHeap, TheNode};

    #[test]
    fn gets_none_if_unavailable() {
        let mut tree = BinaryHeap::new(20);

        tree.insert(TheNode {
            node: "A",
            weight: 1,
        });
        tree.insert(TheNode {
            node: "B",
            weight: 10,
        });
        tree.insert(TheNode {
            node: "O",
            weight: 2,
        });
        tree.insert(TheNode {
            node: "J",
            weight: 26,
        });
        tree.insert(TheNode {
            node: "Q",
            weight: 6,
        });
        tree.insert(TheNode {
            node: "E",
            weight: 9,
        });
        tree.insert(TheNode {
            node: "F",
            weight: 1,
        });

        let common_ancestor = tree.get_common_ancestor("Q", "J");
        assert!(common_ancestor.is_some());
        assert_eq!(common_ancestor.unwrap(), 1);

        let common_ancestor = tree.get_common_ancestor("Q", "F");
        assert!(common_ancestor.is_some());
        assert_eq!(common_ancestor.unwrap(), 0);

        let common_ancestor = tree.get_common_ancestor("J", "B");
        assert!(common_ancestor.is_some());
        assert_eq!(common_ancestor.unwrap(), 1);

        let common_ancestor = tree.get_common_ancestor("J", "F");
        assert!(common_ancestor.is_some());
        assert_eq!(common_ancestor.unwrap(), 0);

        let common_ancestor = tree.get_common_ancestor("J", "W");
        assert!(common_ancestor.is_none());
    }
}
