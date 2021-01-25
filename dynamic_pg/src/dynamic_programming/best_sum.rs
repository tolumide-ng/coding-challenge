/// BEST_SUM
/// Write a function  that takes in target_sum and an array of arguments\\
/// The function should return an array containing the shortest combination of numbers that add up to exactly the target_sum
/// If there is a tie, you may return any of the shortes one

pub mod best_sum {
    use std::cell::RefCell;
    use std::collections::HashMap;

    type CoinType = Option<Vec<usize>>;

    struct BestSum {
        coins: CoinType,
        memo: HashMap<i64, CoinType>,
    }

    impl BestSum {
        fn new(coins: Vec<usize>) -> Self {
            return BestSum {
                coins: Some(coins),
                memo: HashMap::new(),
            };
        }
    }

    pub fn recursive_best_sum(target: i64, coins: Vec<usize>) -> CoinType {
        impl BestSum {
            pub fn get_best_sum(&self, target: i64) -> CoinType {
                if target == 0 {
                    return Some(vec![]);
                }
                if target < 0 {
                    return None;
                }

                let the_best_sum: RefCell<CoinType> = RefCell::new(None);

                let all_coins = self.coins.clone();

                for coin in all_coins.unwrap() {
                    let new_target = target - coin as i64;

                    let value = self.get_best_sum(new_target);

                    match value {
                        Some(mut value) => {
                            value.push(coin);

                            if the_best_sum.borrow().is_none()
                                || (the_best_sum.borrow().as_ref().unwrap().len() > value.len())
                            {
                                *the_best_sum.borrow_mut() = Some(value);
                            }
                        }
                        None => {}
                    }
                }
                return the_best_sum.into_inner();
            }
        }

        let result = BestSum::new(coins);
        return result.get_best_sum(target);
    }

    pub fn memoized_best_sum(target: i64, coins: Vec<usize>) -> CoinType {
        impl BestSum {
            pub fn get_memoized_sum(&mut self, target: i64) -> CoinType {
                if self.memo.contains_key(&target) {
                    let value = self.memo.get(&target);
                    return value.unwrap().clone();
                }
                if target == 0 {
                    return Some(vec![]);
                }

                if target < 0 {
                    return None;
                }

                let the_best_sum: RefCell<CoinType> = RefCell::new(None);

                let all_coins = self.coins.clone();

                for coin in all_coins.unwrap() {
                    let new_target = target - coin as i64;

                    let memoized_result = self.get_memoized_sum(new_target);

                    match memoized_result.clone() {
                        Some(mut value) => {
                            value.push(coin);

                            self.memo.insert(new_target, memoized_result);

                            if the_best_sum.borrow().is_none()
                                || (the_best_sum.borrow().as_ref().unwrap().len() > value.len())
                            {
                                *the_best_sum.borrow_mut() = Some(value);
                            }
                        }

                        None => {}
                    }
                }

                return the_best_sum.into_inner();
            }
        }

        let mut result = BestSum::new(coins);
        return result.get_memoized_sum(target);
    }

    pub fn tabulated_best_sum(target: usize, coins: Vec<usize>) -> CoinType {
        impl BestSum {
            pub fn get_tabulated_sum(&self, target: usize) -> CoinType {
                let mut vec_store: Vec<CoinType> = vec![None; target + 1];

                let all_coins = &self.coins.clone().unwrap();

                vec_store[0] = Some(vec![]);

                for value in 0..=target {
                    for coin in all_coins {
                        let new_target = value + coin;
                        if new_target <= target && vec_store[value].is_some() {
                            let _current_total: usize = vec_store[new_target]
                                .as_ref()
                                .unwrap_or(&vec![0])
                                .iter()
                                .sum();

                            if vec_store[new_target].is_some() {
                                let mut new_variant = vec_store[value].clone().unwrap();
                                new_variant.push(*coin);

                                let curr_variant = vec_store[new_target].clone().unwrap();

                                if new_variant.len() < curr_variant.len() {
                                    vec_store[new_target] = Some(new_variant);
                                }
                            } else if vec_store[new_target].is_none() {
                                let mut new_val = vec_store[value].clone().unwrap();
                                new_val.push(*coin);

                                vec_store[new_target] = Some(new_val);
                            }

                            println!("THE VEC:::::::::::: {:?}", vec_store);
                        }
                    }
                }

                return vec_store[target].clone();
            }
        }
        let result = BestSum::new(coins);
        return result.get_tabulated_sum(target);
    }
}

#[test]
fn test_recursive_best_sum() {
    use best_sum::recursive_best_sum;

    assert_eq!(recursive_best_sum(7, vec![5, 7, 4, 3]), Some(vec![7]));
    assert_eq!(recursive_best_sum(8, vec![2, 3, 5]), Some(vec![5, 3]));
    assert_eq!(recursive_best_sum(8, vec![1, 4, 5]), Some(vec![4, 4]));
}

#[test]
fn test_memoized_best_sum() {
    use best_sum::memoized_best_sum;

    assert_eq!(memoized_best_sum(7, vec![5, 7, 4, 3]), Some(vec![7]));
    assert_eq!(memoized_best_sum(8, vec![2, 3, 5]), Some(vec![5, 3]));
    assert_eq!(memoized_best_sum(8, vec![1, 4, 5]), Some(vec![4, 4]));
    assert_eq!(
        memoized_best_sum(100, vec![5, 20, 30]),
        Some(vec![30, 30, 20, 20])
    );
    assert_eq!(
        memoized_best_sum(100, vec![1, 2, 5, 25, 30]),
        Some(vec![25, 25, 25, 25])
    );
}

#[test]
fn test_tabulated_best_sum() {
    use best_sum::tabulated_best_sum;

    assert_eq!(tabulated_best_sum(7, vec![5, 7, 4, 3]), Some(vec![7]));
    assert!(tabulated_best_sum(8, vec![2, 3, 5]).unwrap().contains(&5));
    assert_eq!(tabulated_best_sum(8, vec![1, 4, 5]), Some(vec![4, 4]));
    assert_eq!(tabulated_best_sum(100, vec![5, 20, 30]).unwrap().len(), 4);
    assert_eq!(
        tabulated_best_sum(100, vec![1, 2, 5, 25, 30]),
        Some(vec![25, 25, 25, 25])
    );
    assert_eq!(tabulated_best_sum(8, vec![2, 3, 5, 4, 8, 4]), Some(vec![8]));
}
