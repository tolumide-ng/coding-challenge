// https://leetcode.com/explore/learn/card/queue-stack/230/usage-stack/1394/

pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<String> = vec![];
    let operands = vec!["+", "-", "*", "/"];

    if tokens.len() < 1 {
        return 0;
    }

    for value in tokens {
        if operands.contains(&value.as_str()) {
            match stack.pop() {
                Some(second_val) => match stack.pop() {
                    Some(first_val) => {
                        let first_int: i32 = first_val.parse().unwrap();
                        let second_int: i32 = second_val.parse().unwrap();

                        if value == String::from("+") {
                            let result = first_int + second_int;
                            stack.push(result.to_string());
                        }

                        if value == String::from("-") {
                            let result = first_int - second_int;
                            stack.push(result.to_string());
                        }

                        if value == String::from("*") {
                            let result = first_int * second_int;
                            stack.push(result.to_string());
                        }

                        if value == String::from("/") {
                            let result = first_int / second_int;
                            stack.push(result.to_string());
                        }
                    }
                    None => {}
                },
                None => {}
            }
        } else {
            stack.push(value);
        }
    }

    return stack[0].parse::<i32>().unwrap();
}

use super::test_wrapper::TestHandler;

fn turn_to_string(the_vec: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();

    for value in the_vec {
        result.push(value.to_string());
    }

    return result;
}

#[cfg(test)]
mod test_rpn_cont {
    use super::*;

    #[test]
    fn test_rpn() {
        let all_test = vec![
            TestHandler::new(turn_to_string(vec!["2", "1", "+", "3", "*"]), 9),
            TestHandler::new(turn_to_string(vec!["4", "13", "5", "/", "+"]), 6),
            TestHandler::new(
                turn_to_string(vec![
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
                ]),
                22,
            ),
        ];
        // assert_eq!(eval_rpn(vec!))

        for test in all_test {
            assert_eq!(eval_rpn(test.input), test.expected);
        }
    }
}
