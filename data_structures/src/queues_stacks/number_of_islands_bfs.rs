use std::{collections::VecDeque, usize};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Position {
    h: usize,
    w: usize,
}

impl Position {
    fn new(h: usize, w: usize) -> Self {
        Position { h, w }
    }
}

fn neighbour_pos(h: usize, w: usize, max_h: usize, max_w: usize) -> Vec<Position> {
    let mut loop_vec: Vec<Position> = Vec::new();

    if h > 0 {
        loop_vec.push(Position::new(h - 1, w));
    }

    if w > 0 {
        loop_vec.push(Position::new(h, w - 1));
    }

    if h + 1 < max_h {
        loop_vec.push(Position::new(h + 1, w));
    }

    if w + 1 < max_w {
        loop_vec.push(Position::new(h, w + 1));
    }

    loop_vec
}

pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut islands: i32 = 0;

    let height: usize = grid.len();
    let width: usize = grid[0].len();

    let mut queue = VecDeque::new();
    let mut visited: Vec<Position> = Vec::new();

    for h in 0..height {
        for w in 0..width {
            if &grid[h][w] == &'1' && !visited.contains(&Position::new(h, w)) {
                queue.push_back(Position::new(h, w));
                visited.push(Position::new(h, w));

                islands += 1;

                while queue.len() > 0 {
                    let queue_length = queue.len();

                    for _value in 0..queue_length {
                        match queue.pop_front() {
                            Some(current) => {
                                visited.push(current);

                                for pos in neighbour_pos(current.h, current.w, height, width) {
                                    if &grid[pos.h][pos.w] == &'1' && !visited.contains(&pos) {
                                        queue.push_back(pos);
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }
    }

    return islands;
}

#[cfg(test)]
mod test_islands_number_bfs_cont {
    use super::*;

    #[test]
    fn test_island_number_bfs() {
        let mut grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];
        assert_eq!(num_islands(&mut grid), 1);

        let mut grid = vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ];

        assert_eq!(num_islands(&mut grid), 3);
    }
}
