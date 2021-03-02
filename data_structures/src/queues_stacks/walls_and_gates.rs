use std::collections::VecDeque;

struct WallsAndGates {
    grid: Vec<Vec<i16>>,
    queue: VecDeque<(usize, usize)>,
}

impl WallsAndGates {
    pub fn new(rooms: Vec<Vec<i16>>) -> Self {
        WallsAndGates {
            grid: rooms,
            queue: VecDeque::new(),
        }
    }

    pub fn get_distance(&mut self) {
        let column_len = self.grid.len();
        let row_len = self.grid[0].len();

        for c in 0..column_len {
            for r in 0..row_len {
                let current = self.grid[c][r];
                if current == 0 {
                    self.queue.push_front((c, r));
                }
            }
        }

        while self.queue.len() > 0 {
            let current = self.queue.pop_front().unwrap();
            self.update_neighbours(current.0, current.1, column_len, row_len);
        }
    }

    // pub fn use_bfs(&mut self, x: usize, y: usize, step: usize) {}

    fn update_neighbours(&mut self, c: usize, r: usize, max_c: usize, max_r: usize) {
        let mut loop_vec = vec![];
        if c > 0 {
            loop_vec.push((c - 1, r));
        }
        if r > 0 {
            loop_vec.push((c, r - 1));
        }
        if c < max_c - 1 {
            loop_vec.push((c + 1, r));
        }
        if r < max_r - 1 {
            loop_vec.push((c, r + 1));
        }
        for (column, row) in loop_vec {
            if (self.grid[c][r] + 1 > self.grid[column][row] && self.grid[column][row] != i16::MIN)
                || self.grid[c][r] == -1
            {
                continue;
            }

            self.grid[column][row] = self.grid[c][r] + 1;
            self.queue.push_back((column, row));
        }
    }
}

#[cfg(test)]
mod test_walls {

    use super::*;

    #[test]
    fn test_walls_and_gates() {
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
