use std::collections::HashMap;

pub fn fib(num: u32) -> u32 {
    if num <= 2 {
        return 1;
    }
    return fib(num - 1) + fib(num - 2);
}

pub fn memoized_fib(num: usize) -> usize {
    struct Fibi {
        memo: HashMap<usize, usize>,
    }

    impl Fibi {
        fn new(num: usize) -> Fibi {
            return Fibi {
                memo: HashMap::with_capacity(num),
            };
        }

        fn get_fibi(&mut self, num: usize) -> usize {
            if num <= 2 {
                return 1;
            }

            if !self.memo.contains_key(&num) {
                let fibi_one = self.get_fibi(num - 1);
                let fibi_two = self.get_fibi(num - 2);

                self.memo.entry(num).or_insert(fibi_one + fibi_two);
            }
            return *self.memo.get(&num).unwrap();
        }
    }

    let mut result = Fibi::new(num);
    return result.get_fibi(num);
}
