pub struct Solution {}

impl Solution {
    pub fn solve_solution(board: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut sudoku = Sudoku::new(board.clone());
        sudoku.solve();

        return sudoku.board;
    }
}

struct Sudoku {
    board: Vec<Vec<char>>,
}

impl Sudoku {
    pub fn new(board: Vec<Vec<char>>) -> Self {
        Sudoku { board }
    }

    pub fn is_possible(&self, y: usize, x: usize, n: char) -> bool {
        // loop through the y axis
        for index in 0..9 {
            if self.board[y][index] == n {
                return false;
            }
        }

        // loop through the x axis
        for index in 0..9 {
            if self.board[index][x] == n {
                return false;
            }
        }

        let grid_x = (x / 3) * 3;
        let grid_y = (y / 3) * 3;

        // loop through the specific grid
        for y_index in 0..3 {
            for x_index in 0..3 {
                if self.board[grid_y + y_index][grid_x + x_index] == n {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn solve(&mut self) {
        // if dot_present {
        for y_index in 0..9 {
            for x_index in 0..9 {
                if self.board[y_index][x_index] == '.' {
                    for n in 1..10 {
                        let int_char = std::char::from_digit(n, 10).unwrap();
                        if self.is_possible(y_index, x_index, int_char) {
                            self.board[y_index][x_index] = int_char;

                            self.solve();

                            let dot_present = self
                                .board
                                .iter()
                                .flatten()
                                .collect::<Vec<&char>>()
                                .contains(&&'.');

                            if dot_present {
                                self.board[y_index][x_index] = '.';
                            }
                        }
                    }
                    return;
                }
            }
        }
    }
    // }
}





#[cfg(test)]
mod test_sudoku_mod {
    use crate::recursion::sudoku_player::Solution;

    #[test]
    fn test_sudoku() {
        let mut input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];

        let output = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];

        assert_eq!(Solution::solve_solution(&mut input), output);
    }
}
