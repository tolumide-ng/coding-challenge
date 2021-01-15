use std::mem;

#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

#[derive(Debug, Clone)]
struct LL {
    head: Option<Box<Node>>,
    tail: Option<Box<Node>>,
    total: usize,
}

impl LL {
    fn new() -> Self {
        return LL {
            head: None,
            tail: None,
            total: 0,
        };
    }

    fn add_at_head(&mut self, value: i32) {
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

    fn add_at_tail(&mut self, value: i32) {
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

    fn get(&self, index: i32) -> i32 {
        if index < 0 || index >= self.total as i32 {
            return -1;
        }

        // if index == 0 && self.total == 0 {
        //     return -1;
        // }

        let mut curr_value = self.head.as_ref().unwrap();
        // println!(
        //     "THE WHOELE TREE BUT YOUR QUERY FIRST {:?},--------- {:#?}",
        //     index, self.head
        // );

        for _ in 0..index {
            if curr_value.next.is_some() {
                curr_value = curr_value.next.as_ref().unwrap();
            }
        }

        // println!(
        //     "WHAT THE CURRENT VALUE STANDS FOR *********** {:#?}",
        //     curr_value
        // );

        return curr_value.value;
    }

    fn add_at_index(&mut self, index: i32, value: i32) {
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

    fn delete_at_index(&mut self, index: i32) {
        if index > self.total as i32 || index < 0 {
            return;
        }

        println!(":::::::::::::::::::::::YOU ESAPED THE OLD CONDITIONAL::::::::::::::::");

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
                    if curr_index == index {
                        match &mut curr.next {
                            Some(the_next) => {
                                let right_hand = the_next.clone().next;
                                curr.next = None;
                                curr.next = right_hand;
                                self.total -= 1;
                                break;
                            }
                            None => {
                                println!("WITHIN THIS BEHE ---------------- {:?}", curr);
                                curr.next = None;
                                println!("WITHIN THIS BEHE AFTEE IMPACT :::::::::----------------::::::: {:?}", curr);

                                self.total -= 1;
                                break;
                            }
                        };
                        // let right_hand = curr.next.clone().unwrap().next;

                        // curr.next = None;
                        // curr.next = right_hand;
                        // self.total -= 1;
                        // break;
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

#[test]
fn singly_ll() {
    let mut list = LL::new();
    list.add_at_head(2);
    println!("EVERYTHING 1------------------->>>>>>>>> {:#?}", list);

    // assert_eq!(list.get(0), 7);
    list.delete_at_index(1);
    println!("EVERYTHING 2------------------->>>>>>>>> {:#?}", list);

    list.add_at_head(2);
    println!("EVERYTHING 3------------------->>>>>>>>> {:#?}", list);

    list.add_at_head(7);
    println!("EVERYTHING 4------------------->>>>>>>>> {:#?}", list);

    list.add_at_head(3);
    println!("EVERYTHING 5------------------->>>>>>>>> {:#?}", list);

    list.add_at_head(2);
    println!("EVERYTHING 6------------------->>>>>>>>> {:#?}", list);

    list.add_at_head(5);
    println!("EVERYTHING 7------------------->>>>>>>>> {:#?}", list);

    // list.add_at_tail(3);
    list.add_at_tail(5);
    println!("EVERYTHING 8------------------->>>>>>>>> {:#?}", list);

    // list.add_at_index(3, 0);
    // list.add_at_index(1, 2);
    list.get(5);
    println!("EVERYTHING 9------------------->>>>>>>>> {:#?}", list);

    list.delete_at_index(6);
    println!("EVERYTHING 10------------------->>>>>>>>> {:#?}", list);

    list.delete_at_index(4);
    println!("EVERYTHING 11------------------->>>>>>>>> {:#?}", list);

    // list.add_at_head(6);
    // println!("EVERYTHING------------------->>>>>>>>> {:#?}", list);

    // list.add_at_tail(4);
    // println!("EVERYTHING------------------->>>>>>>>> {:#?}", list);

    // list.get(4);
    // println!("EVERYTHING------------------->>>>>>>>> {:#?}", list);

    // list.add_at_head(4);
    // println!("EVERYTHING------------------->>>>>>>>> {:#?}", list);

    // list.add_at_index(5, 0);
    // println!("EVERYTHING------------------->>>>>>>>> {:#?}", list);

    // list.add_at_head(6);
    // println!("EVERYTHING------------------->>>>>>>>> {:#?}", list);

    assert_eq!(list.get(0), 22);
    // list.add_at_tail(23);
    // assert_eq!(list.get(2), 23);
    // assert_eq!(list.get(7), -1);
    // // println!("the list>>> {:#?}", list);
    // assert_eq!(list.total, 4);
    // list.delete_at_index(2);
    // // println!(":::::::AFTER THE ACTION::::::: {:#?}", list);
    // assert_eq!(list.total, 4);
}
