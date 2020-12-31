/// BEST_SUM
/// Write a function  that takes in targetSum and an array of arguments\\
/// The function should return an array containing the shortest combination of numbers that add up to exactly the targetSum
/// If there is a tie, you may return any of the shortes one

pub mod best_sum {
    use std::cell::RefCell;

    struct BestSum {
        coins: Option<Vec<usize>>,
    }

    impl BestSum {
        fn new(coins: Vec<usize>) -> Self {
            return BestSum { coins: Some(coins) };
        }
    }

    pub fn recursive_best_sum(target: i64, coins: Vec<usize>) -> Option<Vec<usize>> {
        impl BestSum {
            pub fn get_best_sum(&self, target: i64) -> Option<Vec<usize>> {
                if target == 0 {
                    return Some(vec![]);
                }
                if target < 0 {
                    return None;
                }

                let the_best_sum: RefCell<Option<Vec<usize>>> = RefCell::new(None);

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
}

#[test]
fn test_recursive_best_sum() {
    use best_sum::recursive_best_sum;

    assert_eq!(recursive_best_sum(7, vec![5, 7, 4, 3]), Some(vec![7]));
    assert_eq!(recursive_best_sum(8, vec![2, 3, 5]), Some(vec![5, 3]));
    assert_eq!(recursive_best_sum(8, vec![1, 4, 5]), Some(vec![4, 4]));
}
