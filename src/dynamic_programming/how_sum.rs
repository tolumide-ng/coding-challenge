/// Write a function `howSum(targetSum, numbers)` that takes in a targetSum and an array of numbers as arguments.
/// The function should return an array containing any combination of elements that add up exactly to the targetSum.
/// If there is no combination that adds up to the targetSum it should return null

pub mod how_sum {
    struct HowSum {
        coins: Vec<usize>,
    }

    pub fn recursive_how_sum(targetSum: i64, coins: Vec<usize>) -> Option<Vec<usize>> {
        impl HowSum {
            pub fn new(coins: Vec<usize>) -> HowSum {
                return HowSum { coins };
            }

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
