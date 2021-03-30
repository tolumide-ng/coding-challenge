pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let mut stack = Vec::with_capacity(t.len());

    let mut chart = vec![0; t.len()];

    for index in 0..t.len() {
        if !stack.is_empty() {
            while !stack.is_empty() && t[stack[stack.len() - 1]] < t[index] {
                let original_position = stack[stack.len() - 1];
                chart[original_position] = (index - original_position) as i32;
                stack.pop();
            }
        }

        if index != t.len() - 1 {
            if t[index] >= t[index + 1] {
                stack.push(index);
            } else {
                chart[index] = 1;
            }
        }
        // }
    }

    chart
}

#[cfg(test)]
mod test_daily_temps_cont {
    use crate::queues_stacks::test_wrapper::TestHandler;

    use super::*;

    #[test]
    fn test_daily_temps() {
        let all_tests = vec![
            TestHandler::new(
                vec![73, 74, 75, 71, 69, 72, 76, 73],
                vec![1, 1, 4, 2, 1, 1, 0, 0],
            ),
            TestHandler::new(vec![], vec![]),
            TestHandler::new(vec![0], vec![0]),
            TestHandler::new(vec![-1, 4, 5, -2], vec![1, 1, 0, 0]),
            TestHandler::new(
                vec![77, 77, 77, 77, 77, 41, 77, 41, 41, 77],
                vec![0, 0, 0, 0, 0, 1, 0, 2, 1, 0],
            ),
        ];

        for test in all_tests {
            assert_eq!(daily_temperatures(test.input), test.expected);
        }
    }
}
