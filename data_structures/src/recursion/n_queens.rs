use std::fmt::Debug;
#[derive(Debug)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Queens {
    total: i32,
    positions: Vec<Position>,
    n: i32,
}

pub fn total_n_queens(n: i32) -> i32 {
    let mut queens = Queens::new(n);

    queens.recursive_backtrack(0);

    return queens.total;
}
impl Queens {
    pub fn new(n: i32) -> Self {
        Queens {
            total: 0,
            positions: vec![],
            n,
        }
    }

    pub fn recursive_backtrack(&mut self, row: i32) -> bool {
        let mut can_place = false;

        // loop through the current row
        // if all cells on this row are attackable then we need to move back
        for cell_col in 0..self.n {
            let attackable_position = self.is_attackable(row, cell_col);

            if attackable_position && cell_col == self.n - 1 {
                self.positions.pop();
                return can_place;
            }

            // if !attackable_position && cell_col < self.n {continue;}

            if !attackable_position {
                self.positions.push(Position { row, col: cell_col });

                // drill down so far we are not at the last column
                if row + 1 < self.n {
                    let found_match = self.recursive_backtrack(row + 1);
                    can_place = found_match;
                }
            }

            if self.positions.len() as i32 == self.n {
                self.total += 1;

                self.positions.pop();
            }
        }

        if !can_place {
            self.positions.pop();
        }

        return can_place;
    }

    pub fn is_attackable(&self, row: i32, col: i32) -> bool {
        let mut confirm_is_attackable = true;

        for position in &self.positions {
            // attackable row
            if row == position.row {
                return confirm_is_attackable;
            }

            // attackable column
            if col == position.col {
                return confirm_is_attackable;
            }

            // attackable diagonal1
            if row + col == position.row + position.col {
                return confirm_is_attackable;
            }

            // attackable diagonal2
            if row - col == position.row - position.col {
                return confirm_is_attackable;
            }
        }

        confirm_is_attackable = false;

        return confirm_is_attackable;
    }
}

#[cfg(test)]
mod test_n_queens_mod {
    use crate::recursion::n_queens::total_n_queens;
    #[test]
    fn n_queens_test() {
        assert_eq!(total_n_queens(4), 2);
        assert_eq!(total_n_queens(1), 1);
    }
}
