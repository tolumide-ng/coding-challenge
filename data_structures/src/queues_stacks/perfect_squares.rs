use std::collections::HashMap;

pub fn num_squares(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }

    let mut dic: HashMap<i32, i32> = HashMap::new();

    return bfs(n as i32, n as usize, 0, &mut dic);
}

pub fn bfs(current: i32, n: usize, level: i32, dic: &mut HashMap<i32, i32>) -> i32 {
    if dic.get(&current).is_some() {
        return *dic.get(&current).unwrap();
    }
    if current == 0 {
        return 0;
    }

    if current < 0 {
        return i32::MAX;
    }

    let next_level = level + 1;

    let mut min = i32::MAX;

    for i in 0..n {
        if i * i <= n && i != 0 {
            let mut curr = bfs(current - (i * i) as i32, n, next_level, dic);

            if curr != i32::MAX {
                curr += 1;
            }

            min = std::cmp::min(curr, min);
        }
    }

    dic.insert(current, min);

    return min;
}

struct TestHandler {
    input: i32,
    expected: i32,
}

impl TestHandler {
    pub fn new(input: i32, expected: i32) -> Self {
        TestHandler { input, expected }
    }
}

#[cfg(test)]
mod test_perfect_squares_cont {
    use super::*;

    #[test]
    fn test_perfect_squares() {
        let all_tests = vec![
            TestHandler::new(12, 3),
            TestHandler::new(13, 2),
            TestHandler::new(0, 0),
            TestHandler::new(1, 1),
            TestHandler::new(2, 2),
            TestHandler::new(3, 3),
            TestHandler::new(50, 2),
        ];

        for test in all_tests {
            assert_eq!(num_squares(test.input), test.expected);
        }
    }
}
