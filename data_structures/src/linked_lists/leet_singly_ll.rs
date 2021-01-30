pub type LinkedType = Option<Box<Node>>;

#[derive(Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

#[derive(Default, Debug)]
pub struct MyLinkedList {
    pub head: Option<Box<Node>>,
}

// trait Foo: CloneFoo {};

// trait CloneFoo {
//     fn clone_foo(&self) -> Box<dyn Foo>;
// }

// imp CloneFoo for MyLinkedList {

// }

impl MyLinkedList {
    pub fn new() -> Self {
        Default::default()
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1 */
    /// O(n)
    pub fn get(&self, index: i32) -> i32 {
        let mut curr = match self.head {
            Some(ref a) => a,
            None => return -1,
        };
        let mut idx_curr = 0;
        while idx_curr < index {
            if let Some(ref next) = curr.next {
                curr = next;
                idx_curr += 1;
            } else {
                return -1;
            };
        }
        curr.val
    }

    /// O(1)
    pub fn add_at_head(&mut self, val: i32) {
        self.head = Some(Box::new(Node {
            val,
            next: self.head.take(),
        }))
    }

    /// O(n)
    pub fn add_at_tail(&mut self, val: i32) {
        let mut curr = match self.head {
            Some(ref mut a) => a,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };

        while let Some(ref mut next) = curr.next {
            curr = next;
        }
        curr.next = Some(Box::new(Node { val, next: None }));
    }

    /// O(n)
    fn add_at_index(&mut self, index: i32, val: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });

        let mut idx = 0;
        let mut curr = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = curr.next {
                curr = next;
            }
            idx += 1;
        }

        curr.next = Some(Box::new(Node {
            val,
            next: curr.next.take(),
        }));
        self.head = dummy_head.next;
    }

    /// O(n)
    fn delete_at_index(&mut self, index: i32) {
        let mut dummy_head = Box::new(Node {
            val: 0,
            next: self.head.take(),
        });
        let mut idx = 0;
        let mut curr = &mut dummy_head;
        while idx < index {
            if let Some(ref mut next) = curr.next {
                curr = next;
            }
            idx += 1;
        }
        curr.next = curr.next.take().and_then(|a| a.next);
        self.head = dummy_head.next;
    }
}
