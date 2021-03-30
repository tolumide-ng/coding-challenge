#[derive(Clone, Default, Debug)]
pub struct WallsAndGates {
    grid: Vec<Vec<i16>>,
    column_size: usize,
    row_size: usize,
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Position {
    column: usize,
    row: usize,
}

impl WallsAndGates {
    pub fn new(rooms: Vec<Vec<i16>>) -> Self {
        WallsAndGates {
            column_size: rooms.len(),
            row_size: rooms[0].len(),
            grid: rooms,
        }
    }

    pub fn get_distance(&mut self) {
        for c in 0..self.column_size {
            for r in 0..self.row_size {
                let current = self.grid[c][r];
                if current == 0 {
                    let the_loop: Vec<Position> = self.get_loop(c, r);
                    for position in the_loop {
                        self.get_dfs(position.column, position.row, 1);
                    }
                }
            }
        }
    }

    pub fn get_dfs(&mut self, c: usize, r: usize, step: i16) {
        if c >= self.column_size || r >= self.row_size || self.grid[c][r] == -1 {
            return;
        }

        if self.grid[c][r] == i16::MIN || self.grid[c][r] > step {
            self.grid[c][r] = step;

            let the_loop: Vec<Position> = self.get_loop(c, r);

            for position in the_loop {
                self.get_dfs(position.column, position.row, step + 1);
            }
        }
    }

    pub fn get_loop(&self, c: usize, r: usize) -> Vec<Position> {
        let mut loop_vec = vec![];
        if c > 0 {
            loop_vec.push(Position {
                column: c - 1,
                row: r,
            });
        }

        if r > 0 {
            loop_vec.push(Position {
                column: c,
                row: r - 1,
            });
        }
        if c + 1 < self.column_size {
            loop_vec.push(Position {
                column: c + 1,
                row: r,
            });
        }
        if r + 1 < self.row_size {
            loop_vec.push(Position {
                column: c,
                row: r + 1,
            });
        }

        loop_vec
    }
}

#[cfg(test)]
mod test_walls_dfs {
    use super::*;

    #[test]
    fn test_walls_and_gates_dfs() {
        let grid = vec![
            vec![i16::MIN, -1, 0, i16::MIN],
            vec![i16::MIN, i16::MIN, i16::MIN, -1],
            vec![i16::MIN, -1, i16::MIN, -1],
            vec![0, -1, i16::MIN, i16::MIN],
        ];

        let mut the_grid = WallsAndGates::new(grid);

        the_grid.get_distance();

        let results = &[
            &[3, -1, 0, 1],
            &[2, 2, 1, -1],
            &[1, -1, 2, -1],
            &[0, -1, 3, 4],
        ];

        for x in 0..results.len() {
            for y in 0..results[0].len() {
                assert_eq!(the_grid.grid[x][y], results[x][y]);
            }
        }
    }
}
