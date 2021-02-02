#[derive(Debug, Clone, Copy)]
pub struct Node<T> {
    pub weight: i8,
    pub node: T,
    time_added: u8,
}

#[derive(Debug, Clone)]
pub struct BinaryHeap<T> {
    heap: Vec<Node<T>>,
    max_size: usize,
    size: usize,
    realistic_size: usize,
}

use std::fmt::{Debug, Display};

impl<T> BinaryHeap<T>
where
    T: PartialEq + Eq + Display + Debug,
{
    pub fn new(max_size: usize) -> Self {
        BinaryHeap {
            heap: Vec::with_capacity(max_size),
            max_size,
            size: 0,
            realistic_size: 0,
        }
    }

    pub fn get_right_child(&self, index: usize) -> Option<usize> {
        return None;
    }
}
