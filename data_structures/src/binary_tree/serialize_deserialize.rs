// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

pub type TreeNodeType = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub struct Codec {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl Codec {
    pub fn new() -> Self {
        return Codec {};
    }

    pub fn serialize(&self, root: TreeNodeType) -> String {
        let mut serialized = String::from("");

        if root.is_some() {
            let mut queue: Vec<TreeNodeType> = vec![];

            queue.push(Some(Rc::clone(root.as_ref().unwrap())));

            while queue.len() > 0 {
                // queue.push(curr.unwrap().borrow().val);
                // curr.is_some()
                let curr = queue.remove(0);

                match curr.as_ref() {
                    Some(the_curr) => {
                        match the_curr.as_ref().borrow().left.as_ref() {
                            Some(the_left) => queue.push(Some(Rc::clone(the_left))),
                            None => queue.push(None),
                        }

                        match the_curr.as_ref().borrow().right.as_ref() {
                            Some(the_right) => queue.push(Some(Rc::clone(the_right))),
                            None => queue.push(None),
                        }

                        let curr_str: &str = &the_curr.as_ref().borrow().val.to_string()[..];

                        serialized.push_str(curr_str);
                        serialized.push_str(",");
                    }
                    None => {
                        serialized.push_str("_");
                        serialized.push_str(",");
                    }
                }
            }
        }

        return serialized.to_string();
    }

    pub fn deserialize(&self, data: String) -> TreeNodeType {
        // let mut the_chars = data.chars();
        let mut store = vec![];

        let mut root: TreeNodeType = None;

        if data.len() > 0 {
            let formatted_data: Vec<&str> = data.split(",").collect();

            formatted_data.iter().for_each(|c| {
                if *c != "_" {
                    let v = match c.parse::<i32>() {
                        Ok(the_v) => the_v,
                        _ => 0, // This scenario would never occur so it's safe
                    };

                    let node = TreeNode::new(v);
                    store.push(Some(Rc::new(RefCell::new(node))));
                }

                if *c == "_" {
                    store.push(None);
                }
            });

            let mut curr: TreeNodeType = None;
            println!("{:#?}", curr);
            let mut curr_index = 1;

            // construct the binary tree
            for index in 0..store.len() {
                if store[index].is_some() {
                    curr = Some(Rc::clone(&store[index].as_ref().unwrap()));
                } else {
                    curr = None;
                }

                if root.is_none() {
                    root = Some(Rc::clone(&store[index].as_ref().unwrap()));
                }

                if curr.is_some() {
                    if curr_index < store.len() && store[curr_index].is_some() {
                        curr.as_ref().unwrap().as_ref().borrow_mut().left =
                            Some(Rc::clone(&store[curr_index].as_ref().unwrap()));
                    }
                    if curr_index < store.len() && store[curr_index].is_none() {
                        curr.as_ref().unwrap().as_ref().borrow_mut().left = None;
                    }

                    curr_index += 1;

                    if curr_index < store.len() && store[curr_index].is_some() {
                        curr.as_ref().unwrap().as_ref().borrow_mut().right =
                            Some(Rc::clone(&store[curr_index].as_ref().unwrap()));
                    }
                    if curr_index < store.len() && store[curr_index].is_none() {
                        curr.as_ref().unwrap().as_ref().borrow_mut().right = None;
                    }

                    curr_index += 1;
                }
            }
        }

        return root;
    }
}

// /**
//  * Your Codec object will be instantiated and called as such:
//  * let obj = Codec::new();
//  * let data: String = obj.serialize(strs);
//  * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
//  */
#[cfg(test)]
mod serialize_deserialize_tests {
    use super::Codec;

    #[test]
    fn test_deserialize() {
        let vec =
            "4,-7,-3,_,_,-9,-3,9,-7,-4,_,6,_,-6,-6,_,_,0,6,5,_,9,_,_,-1,-4,_,_,_,-2".to_owned();

        let codec = Codec::new();

        let root = codec.deserialize(vec);

        match root {
            Some(the_root) => {
                assert_eq!(the_root.as_ref().borrow().val, 4);
                assert!(the_root.as_ref().borrow().left.is_some());

                // FIRST BREADTH

                match the_root.as_ref().borrow().left.as_ref() {
                    Some(first_left) => {
                        assert_eq!(first_left.as_ref().borrow().val, -7);
                        assert!(first_left.as_ref().borrow().left.is_none());
                        assert!(first_left.as_ref().borrow().right.is_none());
                    }
                    None => {}
                }

                match the_root.as_ref().borrow().right.as_ref() {
                    Some(first_right) => {
                        assert_eq!(first_right.as_ref().borrow().val, -3);
                        assert!(first_right.as_ref().borrow().left.is_some());
                        assert!(first_right.as_ref().borrow().right.is_some());

                        // SECOND BREADTH

                        match first_right.as_ref().borrow().left.as_ref() {
                            Some(next_left) => {
                                assert_eq!(next_left.as_ref().borrow().val, -9);
                                assert!(next_left.as_ref().borrow().left.is_some());
                                assert!(next_left.as_ref().borrow().right.is_some());
                            }
                            None => {}
                        }

                        match first_right.as_ref().borrow().right.as_ref() {
                            Some(next_right) => {
                                assert_eq!(next_right.as_ref().borrow().val, -3);
                                assert!(next_right.as_ref().borrow().left.is_some());
                                assert!(next_right.as_ref().borrow().right.is_none());
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    #[test]
    fn test_serialize() {
        let vec =
            "4,-7,-3,_,_,-9,-3,9,-7,-4,_,6,_,-6,-6,_,_,0,6,5,_,9,_,_,-1,-4,_,_,_,-2".to_owned();

        let codec = Codec::new();

        let root = codec.deserialize(vec);

        let v = codec.serialize(root);

        println!("v>>>>>> {:#?}", v);

        let new_root = codec.deserialize(v);

        match new_root {
            Some(the_root) => {
                assert_eq!(the_root.as_ref().borrow().val, 4);
                assert!(the_root.as_ref().borrow().left.is_some());

                // FIRST BREADTH

                match the_root.as_ref().borrow().left.as_ref() {
                    Some(first_left) => {
                        assert_eq!(first_left.as_ref().borrow().val, -7);
                        assert!(first_left.as_ref().borrow().left.is_none());
                        assert!(first_left.as_ref().borrow().right.is_none());
                    }
                    None => {}
                }

                match the_root.as_ref().borrow().right.as_ref() {
                    Some(first_right) => {
                        assert_eq!(first_right.as_ref().borrow().val, -3);
                        assert!(first_right.as_ref().borrow().left.is_some());
                        assert!(first_right.as_ref().borrow().right.is_some());

                        // SECOND BREADTH

                        match first_right.as_ref().borrow().left.as_ref() {
                            Some(next_left) => {
                                assert_eq!(next_left.as_ref().borrow().val, -9);
                                assert!(next_left.as_ref().borrow().left.is_some());
                                assert!(next_left.as_ref().borrow().right.is_some());
                            }
                            None => {}
                        }

                        match first_right.as_ref().borrow().right.as_ref() {
                            Some(next_right) => {
                                assert_eq!(next_right.as_ref().borrow().val, -3);
                                assert!(next_right.as_ref().borrow().left.is_some());
                                assert!(next_right.as_ref().borrow().right.is_none());
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            None => {}
        }
    }
}
