mod target_sum {
    #[derive(Debug)]
    struct BruteForce {
        count: usize,
        numbers: Vec<u8>,
        target: i8,
    }

    pub fn brute_force(target: i8, numbers: Vec<u8>) -> usize {
        // let mut count = 0;

        let mut new_brute = BruteForce {
            count: 0,
            numbers,
            target,
        };

        new_brute.get_total(0, 0);

        return new_brute.count;

        // return self.count;
    }

    impl BruteForce {
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
}

#[cfg(test)]
mod test_target_sum_cont {
    use super::*;

    #[test]
    fn test_target_sum() {
        assert_eq!(target_sum::brute_force(3, vec![1, 1, 1, 1, 1]), 5);
    }
}
