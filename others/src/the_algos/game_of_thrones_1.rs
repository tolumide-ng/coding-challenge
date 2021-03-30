// https://www.hackerrank.com/challenges/game-of-thrones/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Possible {
    YES,
    NO,
}

pub fn game_of_thrones_one(s: &str) -> Possible {
    let mut number_of_odds = 0;

    let mut words_dic: HashMap<char, usize> = HashMap::new();

    for (_index, letter) in s.chars().enumerate() {
        if words_dic.get(&letter).is_some() {
            let total = words_dic.get_mut(&letter).unwrap();
            *total += 1;
        } else {
            words_dic.insert(letter, 1);
        }
    }

    for key in words_dic.keys() {
        if words_dic.get(key).unwrap() % 2 != 0 {
            number_of_odds += 1;
        }
    }

    if vec![0, 1].contains(&number_of_odds) {
        return Possible::YES;
    }

    return Possible::NO;
}

#[cfg(test)]
mod test_got_one_cont {
    use super::*;

    #[test]
    fn test_got_one() {
        let inputs = vec!["aaabbbb", "cdefghmnopqrstuvw", "cdcdcdcdeeeef", ""];
        let outputs = vec![Possible::YES, Possible::NO, Possible::YES, Possible::YES];

        for test in 0..inputs.len() {
            assert_eq!(game_of_thrones_one(inputs[test]), outputs[test]);
        }
    }
}
