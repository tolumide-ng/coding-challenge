/// Write a function `howSum(targetSum, numbers)` that takes in a targetSum and an array of numbers as arguments.
/// The function should return an array containing any combination of elements that add up exactly to the targetSum.
/// If there is no combination that adds up to the targetSum it should return null

pub mod how_sum {

    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    struct HowSum {
        coins: Vec<usize>,
        memo: HashMap<i64, Rc<RefCell<Option<Vec<usize>>>>>,
    }

    impl HowSum {
        pub fn new(coins: Vec<usize>) -> HowSum {
            return HowSum {
                coins,
                memo: HashMap::new(),
            };
        }
    }

    pub fn recursive_how_sum(targetSum: i64, coins: Vec<usize>) -> Option<Vec<usize>> {
        impl HowSum {
            pub fn get_recursive_sum(&self, targetSum: i64) -> Option<Vec<usize>> {
                if targetSum == 0 {
                    return Some(Vec::new());
                }
                if targetSum < 0 {
                    return None;
                }
                // BAD CODE
                let all_coins = self.coins.clone();
                for coin in &all_coins {
                    let new_target = targetSum - *coin as i64;
                    let recursive_result = self.get_recursive_sum(new_target);
                    if recursive_result.is_some() {
                        let mut unwrapped_recursive_result = recursive_result.unwrap();
                        unwrapped_recursive_result.push(*coin);
                        return Some(unwrapped_recursive_result);
                    }
                }
                return None;
            }
        }

        let result = HowSum::new(coins);
        return result.get_recursive_sum(targetSum);
    }

    pub fn memoized_how_sum(target_sum: i64, coins: Vec<usize>) -> Option<Vec<usize>> {
        impl HowSum {
            fn get_memoized_sum(&mut self, target_sum: i64) -> Option<Vec<usize>> {
                if self.memo.contains_key(&target_sum) {
                    println!("WENDY'S PLACE {:?}", self.memo.get(&target_sum));
                    // let value = *self.memo.get(&target_sum).unwrap().clone();
                    return *(self.memo.get(&target_sum).unwrap()).borrow();
                    // return value;
                }

                if target_sum == 0 {
                    return Some(Vec::new());
                }

                if target_sum < 0 {
                    return None;
                }

                // BAD CODE
                let all_coins = self.coins.clone();

                for coin in &all_coins {
                    let new_target = target_sum - *coin as i64;
                    let recursive_result = self.get_memoized_sum(new_target);
                    if recursive_result.is_some() {
                        // let mut unwrapped_recursive_result = recursive_result.unwrap();
                        // unwrapped_recursive_result.push(*coin);
                        let result = self.memo.entry(target_sum).or_insert(Rc::new(RefCell::new(recursive_result)));
                        println!("THE RESULT-------------- {:?}", result);

                        // result.borrow_mut().unwrap().push(*coin);
                        result.get_mut().unwrap().push(*coin);

                        // result.as_ref().unwrap().push(*coin);
                        // return *self.memo.get(&target_sum).unwrap().as_ref();
                        let value = *self.memo.get(&target_sum).unwrap();
                        return *value.get_mut();
                    }
                }

                self.memo.entry(target_sum).or_insert(Rc::new(RefCell::new(None)));
                return None;
            }
        }

        let result = HowSum::new(coins);
        return result.get_memoized_sum(target_sum);
    }
}

#[test]
fn test_recursive_how_sum() {
    use how_sum::recursive_how_sum;

    assert!(recursive_how_sum(7, vec![2, 3]).unwrap().len() > 2);
    assert_eq!(
        recursive_how_sum(7, vec![5, 3, 4, 7]).unwrap().contains(&5),
        false
    );
    assert_eq!(recursive_how_sum(7, vec![2, 4]).is_none(), true);
}
