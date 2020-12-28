/// CANSUM
/// Given a target sum and a an array of coins find all the possible combinations of the coin that would result in the target sum\

pub mod can_sum {
    use std::collections::HashMap;

    struct CanSum {
        coins: Vec<u64>,
        memo: HashMap<i64, bool>,
    }

    impl CanSum {
        fn new(coins: Vec<u64>) -> CanSum {
            return CanSum {
                coins,
                memo: HashMap::new(),
            };
        }

        // fn get_can_sum(&self, target: i64, coins: Vec<u64>);
    }

    pub fn recursive_can_sum(target: i64, coins: Vec<u64>) -> bool {
        // TIME COMPLEXITY -- O(n^m)
        // SPACE COMPLEXITY - 0(m)
        impl CanSum {
            fn get_can_sum(&self, target: i64) -> bool {
                if target == 0 {
                    return true;
                }
                if target < 0 {
                    return false;
                }

                for coin in &self.coins {
                    let new_target = target - *coin as i64;
                    if self.get_can_sum(new_target) == true {
                        return true;
                    }
                }
                return false;
            }
        }

        let result = CanSum::new(coins);
        return result.get_can_sum(target);
    }

    // use std::cell::RefCell;
    // use std::rc::Rc;

    pub fn memoized_can_sum(target: i64, coins: Vec<u64>) -> bool {
        // TIME COMPLEXITY -- O(n*m)
        // SPACE COMPLEXITY - 0(m)
        impl CanSum {
            fn get_memoized_sum(&mut self, target: i64) -> bool {
                if self.memo.contains_key(&target) {
                    return *self.memo.get(&target).unwrap();
                }

                if target == 0 {
                    return true;
                }
                if target < 0 {
                    return false;
                }

                // BAD CODE, YES I SHOULD LEARN HOW TO USE REFCELLS AND RCS ON THE COIN
                let coins = self.coins.clone();

                for coin in coins {
                    let new_target = target - coin as i64;

                    if self.get_memoized_sum(new_target) == true {
                        self.memo.entry(target).or_insert(true);
                        return true;
                    }
                }

                self.memo.entry(target).or_insert(false);
                return false;
            }
        }

        let mut result = CanSum::new(coins);
        return result.get_memoized_sum(target);
    }
}

#[test]
fn recursive_cansum_test() {
    use can_sum::recursive_can_sum;
    assert_eq!(recursive_can_sum(7, vec![2, 3]), true);
    assert_eq!(recursive_can_sum(8, vec![2, 3]), true);
    assert_eq!(recursive_can_sum(7, vec![5, 3]), false);
}

fn memoized_cansum_total() {
    use can_sum::memoized_can_sum;
    assert_eq!(memoized_can_sum(7, vec![2, 3]), true);
    assert_eq!(memoized_can_sum(8, vec![2, 3]), true);
    assert_eq!(memoized_can_sum(7, vec![5, 3]), false);
    assert_eq!(memoized_can_sum(300, vec![7, 14]), false);
    assert_eq!(memoized_can_sum(300, vec![7, 3]), true);
}
