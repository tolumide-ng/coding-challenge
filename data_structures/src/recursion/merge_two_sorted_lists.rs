pub mod merge_two_sorted_lists {
    // pub type Node = Option<Box<ListNode>>;
    use crate::recursion::make_ll::{ListNode, Node};

    pub fn merge_approach_two(l1: Node, l2: Node) -> Node {
        match (l1, l2) {
            (Some(mut a), Some(mut b)) => {
                if a.val < b.val {
                    a.next = merge_approach_two(a.next, Some(b));
                    Some(a)
                } else {
                    b.next = merge_approach_two(Some(a), b.next);
                    Some(b)
                }
            }
            (Some(a), _) | (_, Some(a)) => Some(a),
            _ => None,
        }
    }

    pub fn merge_two_lists(l1: Node, l2: Node) -> Node {
        let mut l1 = l1;
        let mut l2 = l2;

        recursive_merge_list(&mut l1, &mut l2)
    }

    pub fn recursive_merge_list(l1: &mut Node, l2: &mut Node) -> Node {
        if l1.is_none() && l2.is_none() {
            return None;
        }

        if l1.is_some() && l2.is_some() {
            let mut the_l1 = std::mem::take(l1);
            let mut the_l2 = std::mem::take(l2);

            if the_l1.as_ref().unwrap().val < the_l2.as_ref().unwrap().val
                || the_l1.as_ref().unwrap().val == the_l2.as_ref().unwrap().val
            {
                // let mut curr = the_l1;

                match &mut the_l1 {
                    Some(the_curr) => {
                        let mut curr = Some(Box::new(ListNode::new(the_curr.val)));

                        let result = recursive_merge_list(&mut the_curr.next, &mut the_l2);

                        let mut new_curr = std::mem::take(&mut curr);

                        match &mut new_curr {
                            Some(the_new_curr) => {
                                the_new_curr.next = result;
                            }
                            None => {}
                        }

                        return new_curr;
                    }
                    None => {}
                }
            }

            if the_l2.as_ref().unwrap().val < the_l1.as_ref().unwrap().val {
                match &mut the_l2 {
                    Some(the_curr) => {
                        let mut curr = Some(Box::new(ListNode::new(the_curr.val)));

                        let result = recursive_merge_list(&mut the_l1, &mut the_curr.next);

                        let mut new_curr = std::mem::take(&mut curr);

                        match &mut new_curr {
                            Some(the_new_curr) => {
                                the_new_curr.next = result;
                            }
                            None => {}
                        }

                        return new_curr;
                    }
                    None => {}
                }
            }
        }

        if l1.is_some() {
            let the_l1 = std::mem::take(l1);

            return the_l1;
        }

        if l2.is_some() {
            let the_l2 = std::mem::take(l2);

            return the_l2;
        }

        return None;
    }
}

#[cfg(test)]
mod merge_two_sorted_list_mod {
    use super::merge_two_sorted_lists::{merge_approach_two, merge_two_lists};
    use crate::recursion::make_ll::make_ll;

    #[test]
    fn test_merge_lists() {
        let l1 = make_ll(vec![1, 2, 4]);
        let l2 = make_ll(vec![1, 3, 4]);

        assert_eq!(merge_two_lists(l1, l2), make_ll(vec![1, 1, 2, 3, 4, 4]));

        // assert_eq!()
    }

    #[test]
    fn test_short_list() {
        let l1 = make_ll(vec![1]);
        let l2 = make_ll(vec![2]);

        assert_eq!(merge_two_lists(l1, l2), make_ll(vec![1, 2]));
    }

    #[test]
    fn test_merge_two_lists_approach_2() {
        let l1 = make_ll(vec![1, 2, 4]);
        let l2 = make_ll(vec![1, 3, 4]);

        assert_eq!(merge_approach_two(l1, l2), make_ll(vec![1, 1, 2, 3, 4, 4]));
    }
}
