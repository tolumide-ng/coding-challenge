use std::cmp;

pub mod grid_traveller {
    use std::collections::HashMap;
    pub fn grid_traveller(m: usize, n: usize) -> usize {
        if m == 1 && n == 1 {
            return 1;
        }
        if m == 0 || n == 0 {
            return 0;
        }
        return grid_traveller(m - 1, n) + grid_traveller(m, n - 1);
    }

    pub fn memoized_grid_traveller(m: usize, n: usize) -> usize {
        struct Grid {
            memo: HashMap<String, usize>,
        }
        impl Grid {
            fn new(num: usize) -> Grid {
                return Grid {
                    memo: HashMap::with_capacity(num),
                };
            }
            fn get_paths(&mut self, m: usize, n: usize) -> usize {
                let the_path = format!("{},{}", m, n);
                if m == 1 && n == 1 {
                    return 1;
                }
                if m == 0 || n == 0 {
                    return 0;
                }
                if !self.memo.contains_key(&the_path) {
                    let path_one = self.get_paths(m - 1, n);
                    let path_two = self.get_paths(m, n - 1);
                    self.memo
                        .entry(the_path.clone())
                        .or_insert(path_one + path_two);
                }
                let s: &str = &the_path[..];
                return *self.memo.get(s).unwrap();
            }
        }
        let mut result = Grid::new(std::cmp::max(m, n));
        return result.get_paths(m, n);
    }
}

#[test]
fn test_grid_traveller() {
    use grid_traveller::*;

    assert_eq!(memoized_grid_traveller(2, 3), 3);
    assert_eq!(memoized_grid_traveller(5, 3), 15);
}
