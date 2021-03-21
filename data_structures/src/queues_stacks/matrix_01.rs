use std::{collections::VecDeque, usize};

#[derive(Debug, Default)]
struct Position {
    x: i32,
    y: i32,
}

pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut dist = vec![vec![std::i32::MAX; matrix[0].len()]; matrix.len()];
    let mut queue: VecDeque<Position> = VecDeque::new();

    for x in 0..matrix.len() {
        for y in 0..matrix[0].len() {
            if matrix[x][y] == 0 {
                queue.push_back(Position {
                    x: x as i32,
                    y: y as i32,
                });

                dist[x][y] = 0;
            }
        }
    }

    let directions: Vec<Position> = vec![
        Position { x: 0, y: 1 },
        Position { x: 0, y: -1 },
        Position { x: 1, y: 0 },
        Position { x: -1, y: 0 },
    ];

    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();

        for position in &directions {
            let new_x = position.x + curr.x;
            let new_y = position.y + curr.y;

            if new_x < 0
                || new_x >= matrix.len() as i32
                || new_y < 0
                || new_y >= matrix[0].len() as i32
            {
                continue;
            }

            if dist[new_x as usize][new_y as usize] > dist[curr.x as usize][curr.y as usize] + 1 {
                dist[new_x as usize][new_y as usize] = dist[curr.x as usize][curr.y as usize] + 1;
                queue.push_back(Position { x: new_x, y: new_y });
            }
        }
    }

    return dist;
}

#[cfg(test)]
mod test_matrix_01_cont {
    use super::*;

    #[test]
    fn test_matrix_01() {
        let input = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let output = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];

        assert_eq!(update_matrix(input), output);

        let input = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let output = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];

        assert_eq!(update_matrix(input), output);

        let input = vec![vec![0], vec![0], vec![0], vec![0], vec![0]];
        let output = vec![vec![0], vec![0], vec![0], vec![0], vec![0]];

        assert_eq!(update_matrix(input), output);
    }
}
