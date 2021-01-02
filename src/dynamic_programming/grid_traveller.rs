pub mod grid_traveller {
    use std::cmp;

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

    pub fn tabulated_grid_traveller(m: usize, n: usize) -> usize {
        let mut two_d_vec: Vec<Vec<usize>> = vec![vec![0; n + 1]; m + 1];

        two_d_vec[1][1] = 1;

        for row in 0..=m {
            for col in 0..=n {
                if row < m {
                    two_d_vec[row + 1][col] = two_d_vec[row + 1][col] + two_d_vec[row][col];
                }
                if col < n {
                    two_d_vec[row][col + 1] = two_d_vec[row][col + 1] + two_d_vec[row][col];
                }
            }
        }

        return two_d_vec[m][n];
    }
}

#[test]
fn test_grid_traveller() {
    use grid_traveller::*;

    assert_eq!(memoized_grid_traveller(2, 3), 3);
    assert_eq!(memoized_grid_traveller(5, 3), 15);
}

#[test]
fn tabulated_grid_traveller() {
    use grid_traveller::tabulated_grid_traveller;

    assert_eq!(tabulated_grid_traveller(3, 3), 6);
    assert_eq!(tabulated_grid_traveller(5, 3), 15);
    assert_eq!(tabulated_grid_traveller(2, 3), 3);
}
