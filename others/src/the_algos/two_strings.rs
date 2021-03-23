#[derive(Debug, PartialEq)]
pub enum TwoStrings {
    YES,
    NO,
}

pub fn two_string(s1: &str, s2: &str) -> TwoStrings {
    let mut possible = TwoStrings::NO;

    let s1_chars: Vec<char> = s1.chars().collect();

    for index in 0..s1_chars.len() {
        if s2.contains(s1_chars[index]) {
            possible = TwoStrings::YES;
        }
    }
    return possible;
}

#[cfg(test)]
mod test_two_strings_cont {
    use super::*;

    #[test]
    fn test_two_strings() {
        let a = "hello";
        let b = "world";

        assert_eq!(two_string(a, b), TwoStrings::YES);

        let a = "hi";
        let b = "world";
        assert_eq!(two_string(a, b), TwoStrings::NO);
    }
}
