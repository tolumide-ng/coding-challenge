use std::usize;

struct Solution {}
pub struct Position {
    column: usize,
    row: usize,
}

pub fn num_islands(grid: &mut Vec<Vec<char>>) -> i32 {
    let mut islands = 0;

    if grid.len() > 0 {
        let column_size = grid.len();
        let row_size = grid[0].len();

        for c in 0..column_size {
            for r in 0..row_size {
                if &grid[c][r] == &'1' {
                    islands += get_dfs(grid, c, r);
                }
            }
        }
    }

    return islands as i32;
}

pub fn get_dfs(grid: &mut Vec<Vec<char>>, c: usize, r: usize) -> usize {
    let max_c = grid.len();
    let max_r = grid[0].len();
    if &grid[c][r] == &'0' || c >= max_c || r >= max_r {
        return 0;
    }

    for position in get_neighbours(max_c, max_r, c, r) {
        grid[c][r] = '0';
        get_dfs(grid, position.column, position.row);
    }

    return 1;
}

pub fn get_neighbours(max_c: usize, max_r: usize, c: usize, r: usize) -> Vec<Position> {
    let mut loop_vec = vec![];

    if c > 0 {
        loop_vec.push(Position {
            column: c - 1,
            row: r,
        })
    }

    if r > 0 {
        loop_vec.push(Position {
            column: c,
            row: r - 1,
        })
    }

    if c + 1 < max_c {
        loop_vec.push(Position {
            column: c + 1,
            row: r,
        })
    }

    if r + 1 < max_r {
        loop_vec.push(Position {
            column: c,
            row: r + 1,
        })
    }

    loop_vec
}

#[cfg(test)]
mod test_islands_number {
    use super::*;

    #[test]
    fn test_island_number() {
        let mut grid = vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ];

        let result = num_islands(&mut grid);
        assert_eq!(result, 1);
    }
}
