use std::mem;

#[derive(Debug, Clone)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
pub struct LL {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    total: usize,
}

impl LL {
    pub fn new() -> Self {
        return LL {
            head: None,
            tail: None,
            total: 0,
        };
    }

    pub fn add_at_head(&mut self, value: i32) {
        // println!("BEFORE---------- ADDING TO RAIL{{{{{{{{ {:#?}", self.head);

        if self.total == 0 {
            self.head = Some(Box::new(Node { value, next: None }));
            self.tail = Some(Box::new(Node { value, next: None }));
        } else {
            let old_head = mem::take(&mut self.head).unwrap();

            self.head = Some(Box::new(Node {
                value,
                next: Some(old_head),
            }));
        }
        // println!("AFTER ADDING TO RAIL{{{{{{{{ {:#?}", self.head);

        self.total += 1;
    }

    pub fn add_at_tail(&mut self, value: i32) {
        match &mut self.head {
            Some(head) => {
                let mut curr = head;

                loop {
                    if curr.next.is_none() {
                        curr.next = Some(Box::new(Node { value, next: None }));
                        break;
                    }

                    curr = match &mut curr.next {
                        Some(x) => x,
                        None => break,
                    }
                }
            }
            None => {
                self.head = Some(Box::new(Node { value, next: None }));
                self.tail = Some(Box::new(Node { value, next: None }));
            }
        }
        self.total += 1;
    }

    pub fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.total as i32 {
            return -1;
        }

        // if index == 0 && self.total == 0 {
        //     return -1;
        // }

        let mut curr_value = self.head.as_ref().unwrap();

        for _ in 0..index {
            if curr_value.next.is_some() {
                curr_value = curr_value.next.as_ref().unwrap();
            }
        }

        return curr_value.value;
    }

    pub fn add_at_index(&mut self, index: i32, value: i32) {
        if index > self.total as i32 || index < 0 {
            return;
        }

        match &mut self.head {
            Some(head) => {
                let mut curr = head;

                let mut curr_index = 0;

                if index == 0 {
                    self.head = Some(Box::new(Node {
                        value,
                        next: Some(curr.clone()),
                    }));
                    self.total += 1;
                    return;
                }

                loop {
                    curr_index += 1;

                    if curr_index == index {
                        let right_heand = curr.next.clone();
                        curr.next = Some(Box::new(Node {
                            value,
                            next: right_heand,
                        }));
                        self.total += 1;
                        break;
                    }

                    curr = match &mut curr.next {
                        Some(x) => x,
                        None => break,
                    };
                }
            }
            None => {
                if index == self.total as i32 {
                    self.head = Some(Box::new(Node { value, next: None }));
                    self.total += 1;
                }
            }
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if index >= self.total as i32 || index < 0 {
            return;
        }

        match &mut self.head {
            Some(head) => {
                let mut curr = head;
                let mut curr_index = 0;

                if index == 0 {
                    let right_hand = curr.next.clone();
                    self.head = right_hand;
                    self.total -= 1;
                    return;
                }

                loop {
                    if curr_index == (index - 1) {
                        match &mut curr.next {
                            Some(the_next) => {
                                let right_hand = the_next.clone().next;
                                curr.next = None;
                                curr.next = right_hand;

                                self.total -= 1;
                                break;
                            }
                            None => {
                                curr.next = None;

                                self.total -= 1;
                                break;
                            }
                        };
                    }
                    curr_index += 1;
                    curr = match &mut curr.next {
                        Some(x) => x,
                        None => break,
                    }
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod test_singly_ll_mod {
    #[test]
    fn singly_ll() {
        use super::LL;

        let mut list = LL::new();
        list.add_at_head(2);
        assert_eq!(list.get(1), -1);
        list.delete_at_index(1);
        list.add_at_head(2);
        list.add_at_head(7);
        assert_eq!(list.get(1), 2);
        list.add_at_head(3);
        list.add_at_head(2);
        list.add_at_head(5);
        list.add_at_tail(5);
        assert_eq!(list.get(list.total as i32 - 1), 5);
        list.get(5);
        list.delete_at_index(6);
        assert_eq!(list.get(0), 5);
        assert_eq!(list.get(list.total as i32 - 1), 2);
        list.delete_at_index(4);
        list.delete_at_index(0);
        assert_eq!(list.get(0), 2);
        list.add_at_index(5, 55);
        assert_ne!(list.get(list.total as i32 - 1), 55);
        assert_eq!(list.get(list.total as i32 - 1), 2);
    }
}
