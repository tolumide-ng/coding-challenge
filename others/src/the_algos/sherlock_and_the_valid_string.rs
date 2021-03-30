use std::cmp::PartialEq;
use std::collections::HashMap;
/// Sherlock considers a string to be valid if all characters of the string appear the same number of times.
/// It is also valid if he can remove just  character at  index in the string, and the remaining characters will occur the same number of times. Given a string ,
/// determine if it is valid. If so, return YES, otherwise return NO.
/// https://www.hackerrank.com/challenges/sherlock-and-valid-string/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=strings

#[derive(Debug, PartialEq)]
pub enum Answer {
    Yes,
    No,
}

#[derive(Debug, Default)]
pub struct MinMax {
    min_value: usize,
    min_count: usize,
    max_value: usize,
    max_count: usize,
}

pub fn valid_string(input: &str) -> Answer {
    let mut is_valid = Answer::No;
    let mut store: HashMap<char, usize> = HashMap::new();

    for char in input.chars().into_iter() {
        let current = store.entry(char).or_insert(0);
        *current += 1;
    }

    let mut state: MinMax = Default::default();

    let all_counts: Vec<usize> = store.into_iter().map(|(_value, count)| count).collect();
    state.min_value = *all_counts.iter().min().unwrap();
    state.max_value = *all_counts.iter().max().unwrap();

    all_counts.iter().for_each(|&val| {
        if val == state.min_value {
            state.min_count += 1;
        }
        if val == state.max_value {
            state.max_count += 1;
        }
    });

    if (state.min_value == state.max_value)
        && state.max_count == all_counts.len()
        && state.min_count == all_counts.len()
    {
        is_valid = Answer::Yes;
    }

    let value_difference = state.min_value as i32 - state.max_value as i32;
    let expected_difference: Vec<i32> = vec![1, -1];

    if (state.min_value != state.max_value)
        && (state.min_count + state.max_count == all_counts.len())
        && (expected_difference.contains(&value_difference))
        && (vec![state.min_count, state.max_count].contains(&1))
    {
        is_valid = Answer::Yes;
    }

    return is_valid;
}

#[cfg(test)]
mod valid_string {
    use super::*;

    #[test]
    fn check_string_valid() {
        assert_eq!(valid_string("toluumiddde"), Answer::No);
        assert_eq!(valid_string("abc"), Answer::Yes);
        assert_eq!(valid_string("abcc"), Answer::Yes);
        assert_eq!(valid_string("abcdefghhgfedecba"), Answer::Yes);
    }

    #[test]
    fn check_invalid_string() {
        assert_eq!(valid_string("abccc"), Answer::No);
        assert_eq!(valid_string("abbccc"), Answer::No);
        assert_eq!(valid_string("aabbcd"), Answer::No);
        assert_eq!(valid_string("aabbccddeefghi"), Answer::No);
    }
}
