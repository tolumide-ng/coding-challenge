struct Position {
    row: usize,
    col: usize,
}

struct Queens {
    total: i32,
    positions: vec![Position],
    n: i32,
}

pub fn total_n_queens(n: i32) -> i32 {
    let queens = Queens::new(n);

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

    pub fn recursive_backtrack(&mut self, row: usize, col: usize) -> bool {
        let mut result = false;

        while row < self.n as usize {
            self.positions.push(Position { row, col });
        }

        return result;
    }

    pub fn confirm_is_attackable(&self, row: usize, col: usize) -> bool {
        let mut is_attackable = false;

        for attackables in 0..self.positions {
            // attackable row

            // attackable column

            // attackable diagonal1

            // attackable diagonal2
        }

        return is_attackable;
    }
}
