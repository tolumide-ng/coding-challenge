pub mod target_sum {
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct TargetSum {
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

        return new_brute.count;

        // return self.count;
    }

    impl TargetSum {
        pub fn get_total(&mut self, sum: i8, index: usize) {
            if index >= self.numbers.len() {
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
        pub fn get_memoized_total(&mut self, sum: i8, index: usize) -> i8 {
            let mut ans = 0;
            println!("ans {}", ans);

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

    pub fn get_target_sum(target: i32, numbers: Vec<i32>) -> i32 {
        let sum: i32 = numbers.iter().sum();

        if target > sum || ((sum + target) % 2) != 0 {
            return 0;
        }

        let new_target: i32 = (target + sum) / 2;

        let mut dp: Vec<i32> = vec![0; new_target as usize + 1];

        dp[0] = 1;

        for number in numbers {
            for j in ((number)..=new_target).rev() {
                let index = j - number;

                if index >= 0 {
                    dp[j as usize] = dp[j as usize] + dp.get(index as usize).unwrap();
                }
            }
        }

        return dp[new_target as usize];
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
        assert_eq!(target_sum::memoized(3, vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(target_sum::memoized(1, vec![0, 1]), 2);
    }

    #[test]
    fn test_dynamic_target_sum() {
        assert_eq!(target_sum::get_target_sum(3, vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(target_sum::get_target_sum(1, vec![0, 1]), 2);
    }
}
