/// CANSUM
/// Given a target sum and a an array of coins find all the possible combinations of the coin that would result in the target sum\

pub mod can_sum {
    struct CanSum {
        coins: Vec<u64>,
    }

    pub fn recursive_can_sum(target: i64, coins: Vec<u64>) -> bool {
        impl CanSum {
            fn new(coins: Vec<u64>) -> CanSum {
                return CanSum { coins };
            }

            fn get_can_sum(&self, target: i64) -> bool {
                if target == 0 {
                    return true;
                }
                if target < 0 {
                    return false;
                }

                // if *coin < target {
                //     return false;
                // }
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
}

#[test]
fn recursive_cansum_test() {
    use can_sum::recursive_can_sum;
    assert_eq!(recursive_can_sum(7, vec![2, 3]), true);
    assert_eq!(recursive_can_sum(8, vec![2, 3]), true);
    assert_eq!(recursive_can_sum(7, vec![5, 3]), false);
}
