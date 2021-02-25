#[derive(Debug)]
struct MyCircularQueue {
    queue: Vec<i32>,
    head: Option<usize>,
    tail: Option<usize>,
    capacity: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        MyCircularQueue {
            queue: vec![-1; k as usize],
            head: None,
            tail: None,
            capacity: k as usize,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }

        if self.head.is_some() && self.tail.is_some() {
            let tail = self.tail.unwrap();

            if tail == self.capacity - 1 {
                self.tail = Some(0);
                self.queue[0] = value;
            } else {
                let new_tail = tail + 1;
                self.tail = Some(new_tail);
                self.queue[new_tail] = value;
            }
        }

        if self.head.is_none() && self.tail.is_none() {
            self.head = Some(0);
            self.tail = Some(0);
            self.queue[0] = value;
        }

        return true;
    }

    fn de_queue(&mut self) -> bool {
        if self.head.is_some() && self.tail.is_some() {
            let head = self.head.unwrap();
            let tail = self.tail.unwrap();

            self.queue[head] = -1;
            if tail == head {
                self.head = None;
                self.tail = None;
            } else if head != self.capacity - 1 {
                self.head = Some(head + 1);
            } else if head == self.capacity - 1 {
                self.head = Some(0);
            }

            return true;
        }

        return false;
    }

    fn front(&self) -> i32 {
        if self.head.is_some() {
            let head = self.head.unwrap();
            return match self.queue.get(head) {
                Some(value) => *value,
                None => -1,
            };
        }
        return -1;
    }

    fn rear(&self) -> i32 {
        if self.tail.is_some() {
            let tail = self.tail.unwrap();
            return match self.queue.get(tail) {
                Some(value) => *value,
                None => -1,
            };
        }
        return -1;
    }

    fn is_empty(&self) -> bool {
        if self.head.is_none() && self.tail.is_none() {
            return true;
        }

        return false;
    }

    fn is_full(&self) -> bool {
        return !self.queue.contains(&-1);
    }
}

#[cfg(test)]
mod test_queue {
    use super::*;
    #[test]
    fn test_queue_edge_case() {
        let mut queue = MyCircularQueue::new(8);
        queue.en_queue(3);
        queue.en_queue(9);
        queue.en_queue(5);

        queue.en_queue(0);

        assert_eq!(queue.de_queue(), true);

        assert_eq!(queue.de_queue(), true);

        assert_eq!(queue.is_empty(), false);
        assert!(!queue.is_empty());
        assert_eq!(queue.rear(), 0);
        assert_eq!(queue.de_queue(), true);
        assert_eq!(queue.rear(), 0);

        assert_eq!(queue.front(), 0);
    }

    #[test]
    fn test_enqeue_rear() {
        let mut queue = MyCircularQueue::new(3);
        queue.en_queue(1);
        queue.en_queue(2);
        queue.en_queue(3);
        queue.en_queue(4);
        queue.rear();
        assert_eq!(queue.is_full(), true);

        queue.de_queue();
        queue.en_queue(4);

        (queue.rear());
    }
}
