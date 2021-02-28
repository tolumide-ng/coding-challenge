// Given a stream of integers and a window size, calculate the moving avergae of all the integers in the sliding window

use crate::queues_stacks::circular_queue::MyCircularQueue;

impl MyCircularQueue {
    pub fn next(&mut self, value: i32) -> f32 {
        let mut val = value as f32;

        if !self.is_full() {
            self.count += 1;
        }

        if self.is_full() {
            let val_to_remove = self.front();
            self.sum -= val_to_remove;
            self.de_queue();
        }

        self.en_queue(value);
        self.sum += value;

        val = (self.sum as f32 / self.count as f32) as f32;

        return val;
    }
}

#[cfg(test)]
mod test_circular_queue {
    use super::*;

    #[test]
    pub fn test_cq() {
        let mut queue = MyCircularQueue::new(3);

        assert_eq!(queue.next(1), 1.0);
        assert_eq!(queue.next(10), 5.5);
        assert_eq!(queue.next(3), (1.0 + 10.0 + 3.0) / 3.0);
        assert_eq!(queue.next(5), (10.0 + 3.0 + 5.0) / 3.0);
    }
}
