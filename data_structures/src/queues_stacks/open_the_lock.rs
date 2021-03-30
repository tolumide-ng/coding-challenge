use std::collections::{HashSet, VecDeque};

pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut output = 0;

    let start = "0000";

    if deadends.contains(&start.to_string()) {
        return -1;
    }

    if target == start {
        return output;
    }
    let mut visited = HashSet::new();

    let mut found = false;

    let mut queue = VecDeque::new();

    queue.push_back(start.to_string());

    while queue.len() > 0 {
        let queue_len = queue.len();

        for _content in 0..queue_len {
            match queue.pop_front() {
                Some(current) => {
                    if !deadends.contains(&current.to_string()) && !visited.contains(&current) {
                        visited.insert(current.clone());
                        let children = get_children(current.as_str());

                        for child in children {
                            queue.push_back(child.clone());

                            if target == *child {
                                found = true;
                                queue.drain(..);

                                break;
                            }
                        }
                    }
                }
                None => {}
            }
        }
        output += 1;
    }

    if !found {
        return -1;
    }

    return output;
}

pub fn get_children(current: &str) -> Vec<String> {
    let mut vec_store = vec![];

    for index in 0..current.len() {
        let left = &current[..index];
        let mut right = &current[index..];
        let curr = &right[..1];

        if right.len() > 1 {
            right = &right[1..];
        } else {
            right = "";
        }

        for up_down in &[1, -1] {
            let new_current = curr.to_string().parse::<i32>().unwrap();
            let mut new_value = new_current + up_down;

            if new_value == 10 {
                new_value = 0;
            }

            if new_value == -1 {
                new_value = 9
            }

            let new_total = format!("{}{}{}", left, new_value, right);

            vec_store.push(new_total);
        }
    }

    vec_store
}

#[derive(Debug)]
pub struct Handler {
    deadlocks: Vec<String>,
    target: String,
    expected: i32,
}

impl Handler {
    pub fn new(deadlocks: Vec<&str>, target: &str, expected: i32) -> Self {
        let deadlocks: Vec<String> = deadlocks.iter().map(|x| String::from(*x)).collect();

        Handler {
            deadlocks,
            target: String::from(target),
            expected,
        }
    }
}

#[cfg(test)]
mod test_open_lock {
    use super::*;

    #[test]
    fn test_the_open_lock() {
        let all_tests = vec![
            Handler::new(vec!["0201", "0101", "0102", "1212", "2002"], "0202", 6),
            Handler::new(vec!["8888"], "0009", 1),
            Handler::new(
                vec![
                    "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
                ],
                "8888",
                -1,
            ),
            Handler::new(vec!["0000"], "8888", -1),
        ];

        for test in all_tests {
            assert_eq!(open_lock(test.deadlocks, test.target), test.expected)
        }
    }
}
