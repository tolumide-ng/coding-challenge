pub fn is_valid(s: String) -> bool {
    let all_str: Vec<&str> = s.split("").collect();
    let mut stack = vec![];

    if s.len() > 0 && s.len() % 2 == 0 {
        for x in all_str {
            if x.len() > 0 {
                if *&["}", "]", ")"].contains(&x) && stack.is_empty() {
                    return false;
                }

                if *&["{", "[", "("].contains(&x) {
                    stack.push(x);
                } else {
                    let last_elem = stack[stack.len() - 1];

                    if last_elem == "{" && x == "}" {
                        stack.pop();
                        continue;
                    }

                    if last_elem == "(" && x == ")" {
                        stack.pop();
                        continue;
                    }

                    if last_elem == "[" && x == "]" {
                        stack.pop();
                        continue;
                    }

                    return false;
                }
            }
        }
    } else {
        return false;
    }

    stack.is_empty()
}

#[cfg(test)]
mod test_valid_parentheses_cont {
    pub struct TestHandler {
        input: String,
        output: bool,
    }
    impl TestHandler {
        pub fn new(input: &str, output: bool) -> Self {
            TestHandler {
                input: String::from(input),
                output,
            }
        }
    }

    use super::*;

    #[test]
    fn test_valid_parentheses() {
        let all_tests = vec![
            TestHandler::new("()[]{", false),
            TestHandler::new("", false),
            TestHandler::new("(]", false),
            TestHandler::new("([)]", false),
            TestHandler::new("{[]}", true),
            TestHandler::new("[{[]}({{[]}})", false),
            TestHandler::new("()))", false),
            TestHandler::new("((((((((((((((())))))))))))))))", false),
            TestHandler::new("((((((((((((((((((", false),
            TestHandler::new("(((((((((((((((((()))[]{})))", false),
        ];

        for test in all_tests {
            assert_eq!(is_valid(test.input), test.output);
        }
    }
}
