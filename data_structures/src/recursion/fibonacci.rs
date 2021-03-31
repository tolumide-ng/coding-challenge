pub mod fib {
    use std::collections::HashMap;

    pub fn fib_fast(n: i32) -> i32 {
        if n < 2 {
            return n;
        }

        let mut nums = vec![0, 1];

        for _i in 1..n {
            let new_sum = nums[0] + nums[1];
            nums[0] = nums[1];
            nums[1] = new_sum;
        }

        nums[1]
    }

    #[derive(Default)]
    struct FibMemo {
        store: HashMap<i32, i32>,
    }

    pub fn get_fib_memo(n: i32) -> i32 {
        let mut result = FibMemo::new();

        result.fib_memo(n);

        *result.store.get(&n).unwrap()
    }

    impl FibMemo {
        pub fn new() -> Self {
            Default::default()
        }

        pub fn fib_memo(&mut self, n: i32) -> i32 {
            if n < 2 {
                return n;
            }

            if self.store.get(&n).is_some() {
                return *self.store.get(&n).unwrap();
            }

            let result = self.fib_memo(n - 1) + self.fib_memo(n - 2);
            self.store.insert(n, result);

            return result;
        }
    }
}

#[cfg(test)]
mod test_fibonacci_mod {
    use crate::recursion::fibonacci::fib::{fib_fast, get_fib_memo};

    #[test]
    fn test_fib_fast() {
        assert_eq!(fib_fast(2), 1);
        assert_eq!(fib_fast(3), 2);
        assert_eq!(fib_fast(4), 3);
        assert_eq!(fib_fast(7), 13);
    }

    #[test]
    fn test_fib_memo() {
        assert_eq!(get_fib_memo(2), 1);
        assert_eq!(get_fib_memo(3), 2);
        assert_eq!(get_fib_memo(4), 3);
        assert_eq!(get_fib_memo(7), 13);
    }
}
