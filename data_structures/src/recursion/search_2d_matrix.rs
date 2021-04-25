pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].len() < 1 {
        return false;
    }
    let row_end: i32 = (matrix.len() - 1) as i32;
    let column_end: i32 = (matrix[0].len() - 1) as i32;

    return find_target(0, row_end, 0, column_end, &matrix, target);
}

struct Position {
    r_s: i32,
    r_e: i32,
    c_s: i32,
    c_e: i32,
}

pub fn find_target(
    row_start: i32,
    row_end: i32,
    column_start: i32,
    column_end: i32,
    matrix: &Vec<Vec<i32>>,
    target: i32,
) -> bool {
    if row_end > (matrix.len() - 1) as i32
        || row_start < 0
        || column_end > (matrix[0].len() - 1) as i32
        || column_start < 0
        || row_start > row_end
        || column_start > column_end
    {
        return false;
    }

    if row_start == row_end
        && column_start == column_end
        && matrix[row_start as usize][column_start as usize] != target
    {
        return false;
    }

    let pivot_row = ((row_start + row_end) / 2) as usize;
    let pivot_col = ((column_start + column_end) / 2) as usize;

    let all_points: Vec<Position> = vec![
        Position {
            // top-right
            c_s: (pivot_col + 1) as i32,
            r_s: row_start,
            r_e: pivot_row as i32,
            c_e: column_end,
        },
        Position {
            // bottom-left
            c_s: column_start,
            c_e: pivot_col as i32,
            r_s: (pivot_row + 1) as i32,
            r_e: row_end,
        },
        Position {
            // bottom-right
            c_s: (pivot_col + 1) as i32,
            c_e: column_end,
            r_s: (pivot_row + 1) as i32,
            r_e: row_end,
        },
        Position {
            c_s: column_start,
            c_e: pivot_col as i32,
            r_s: row_start,
            r_e: pivot_row as i32,
        },
    ];

    if matrix[pivot_row][pivot_col] == target {
        return true;
    }

    for bound in all_points {
        if find_target(bound.r_s, bound.r_e, bound.c_s, bound.c_e, &matrix, target) {
            return true;
        }
    }

    return false;
}

use std::cmp::Ordering::{Equal, Greater, Less};

pub fn search_the_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut rp, mut cp) = (0, matrix[0].len() - 1);
    while matrix.get(rp).is_some() && matrix[rp].get(cp).is_some() {
        match matrix[rp][cp].cmp(&target) {
            Equal => {
                return true;
            }
            Less => {
                rp += 1;
            }
            Greater => {
                cp -= 1;
            }
        }
    }
    false
}

#[cfg(test)]
mod test_search_matrix_mod {
    use crate::recursion::search_2d_matrix::search_matrix;
    #[test]
    fn test_search_matrix() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30],
        ];

        assert_eq!(search_matrix(matrix, 30), true);
    }

    #[test]
    fn test_search_empty_matrix() {
        let matrix = vec![vec![]];

        assert_eq!(search_matrix(matrix, 30), false);
    }
}
