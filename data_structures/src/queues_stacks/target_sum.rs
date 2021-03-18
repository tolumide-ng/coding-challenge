mod target_sum {
    use std::{collections::HashMap, default};

    #[derive(Debug, Default)]
    struct TargetSum {
        count: usize,
        numbers: Vec<u8>,
        target: i8,
        memo: HashMap<(usize, i8), i8>,
    }

    pub fn brute_force(target: i8, numbers: Vec<u8>) -> usize {
        // let mut count = 0;

        let mut new_brute = TargetSum {
            count: 0,
            numbers,
            target,
            ..Default::default()
        };

        new_brute.get_total(0, 0);

        println!("what to return !!!!!!!!!!! {:#?}", new_brute.count);

        return new_brute.count;

        // return self.count;
    }

    impl TargetSum {
        fn get_total(&mut self, sum: i8, index: usize) {
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

            self.get_total(sum - self.numbers[index] as i8, index + 1);
        }
    }

    pub fn memoized(target: i8, numbers: Vec<u8>) -> usize {
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
            let mut ans = 0;

            match self.memo.get(&(index, sum)) {
                Some(content) => return *content,
                None => {
                    if index == self.numbers.len() {
                        if sum == self.target {
                            return 1;
                        }

                        if sum != self.target {}
                        return 0;
                    }

                    let cumulative = self
                        .get_memoized_total(sum + self.numbers[index] as i8, index + 1)
                        + self.get_memoized_total(sum - self.numbers[index] as i8, index + 1);

                    self.memo.insert((index, sum), cumulative);

                    ans = cumulative;
                    self.count = ans as usize;
                }
            }

            return ans;
        }
    }
}

#[cfg(test)]
mod test_target_sum_cont {
    use super::*;

    #[test]
    fn test_target_sum() {
        assert_eq!(target_sum::brute_force(3, vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(target_sum::brute_force(1, vec![0, 1]), 2);
    }

    #[test]
    fn test_memoized_target_sum() {
        // assert_eq!(target_sum::memoized(3, vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(target_sum::memoized(1, vec![0, 1]), 2);
    }
}
