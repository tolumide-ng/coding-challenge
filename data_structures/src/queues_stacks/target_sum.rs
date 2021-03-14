mod target_sum {
    use std::{collections::HashMap, default};

    #[derive(Debug, Default)]
    struct TargetSum {
        count: usize,
        numbers: Vec<u8>,
        target: i8,
        memo: HashMap<String, i8>,
    }

    pub fn brute_force(target: i8, numbers: Vec<u8>) -> usize {
        // let mut count = 0;

        let mut new_brute = TargetSum {
            count: 0,
            numbers,
            target,
            ..Default::default()
        };
        println!("WANNNNEEEEMMMMMWANNNNEEEEMMMMMWANNNNEEEEMMMMMWANNNNEEEEMMMMMWANNNNEEEEMMMMMWANNNNEEEEMMMMMWANNNNEEEEMMMMM");

        new_brute.get_total(0, 0);

        return new_brute.count;

        // return self.count;
    }

    impl TargetSum {
        fn get_total(&mut self, sum: i8, index: usize) {
            println!("here {}, {}", sum - self.numbers[index] as i8, index + 1);
            println!("here {}, {}", sum + self.numbers[index] as i8, index + 1);

            if index >= self.numbers.len() {
                // println!("HERE WITH {:#?}", sum);
                return;
            }

            if index == self.numbers.len() - 1 {
                if sum - self.numbers[index] as i8 == self.target
                    || sum + self.numbers[index] as i8 == self.target
                {
                    self.count += 1;
                    return;
                }
            }

            self.get_total(sum + self.numbers[index] as i8, index + 1);
            // println!(
            //     "SUM INDEX {:#?}--{:#?} {:#?}",
            //     sum,
            //     index,
            //     sum + self.numbers[index] as i8
            // );
            // println!(
            //     "SUM INDEX {:#?}--{:#?} {:#?}",
            //     sum,
            //     index,
            //     sum - self.numbers[index] as i8
            // );

            self.get_total(sum - self.numbers[index] as i8, index + 1);
        }
    }

    pub fn memoized(target: i8, numbers: Vec<u8>) -> usize {
        let mut count = 0;
        let mut new_memoized = TargetSum {
            count: 0,
            numbers,
            target,
            memo: HashMap::new(),
        };

        new_memoized.get_memoized_total(0, 0);

        return new_memoized.count;
    }

    impl TargetSum {
        fn get_memoized_total(&mut self, sum: i8, index: usize) -> i8 {
            let key = format!("{} {}", index, sum);
            // if index >= self.numbers.len() {
            //     // return i8::MIN;
            //     return 0;
            // }
            self.memo.insert(key.clone(), sum);

            if index == self.numbers.len() {
                // self.memo.insert(key, 1);

                if sum == self.target {
                    self.count += 1;
                    println!("SUM IS TARGET");

                    return 1;
                } else {
                    return 0;
                }
            }

            // while index + 1 < self.numbers.len() {

            let value_in_memo = self.memo.get(&key);

            if value_in_memo.is_some() {
                // println!("RETURNING FROM HERE");
                return *value_in_memo.unwrap();
            }

            let minus = self.get_memoized_total(sum - self.numbers[index] as i8, index + 1);
            let add = self.get_memoized_total(sum + self.numbers[index] as i8, index + 1);

            // println!(
            //     "A TRUTHY HAS BEEN FOUNG {}",
            //     index == self.numbers.len() - 1
            // );

            // println!(
            //     "-----------------------the add {} and the minus {}-----------------------",
            //     add, minus
            // );

            // println!("THE MEMO {:#?}", self.memo);
            // }

            self.memo.insert(key.clone(), add + minus);

            println!("the memo {:#?}", self.memo);

            return *self.memo.get(&key).unwrap();

            // return *self.memo.get(&key).unwrap();
        }
    }
}

#[cfg(test)]
mod test_target_sum_cont {
    use super::*;

    #[test]
    fn test_target_sum() {
        // assert_eq!(target_sum::brute_force(3, vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(target_sum::brute_force(1, vec![0, 1]), 2);
    }

    #[test]
    fn test_memoized_target_sum() {
        // assert_eq!(target_sum::memoized(3, vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(target_sum::memoized(1, vec![0, 1]), 2);
    }
}
