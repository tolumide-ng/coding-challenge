use std::collections::{HashMap, VecDeque};

pub fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }

    // Create keyboard with digits combo

    let letters = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    let mut dic: HashMap<u32, Vec<String>> = HashMap::new();

    let mut total_string = 0;
    let mut curr_index = 2;

    for alphabet in letters {
        match total_string {
            0 => {
                dic.insert(curr_index, vec![alphabet.to_string()]);
                total_string += 1;
            }
            _ => {
                dic.get_mut(&curr_index).unwrap().push(alphabet.to_string());
                total_string += 1;

                if curr_index == 7 || curr_index == 9 && total_string == 3 {}

                let extra_letters = vec![7, 9];

                if total_string == 3 && !extra_letters.contains(&curr_index) {
                    total_string = 0;
                    curr_index += 1;
                }

                if extra_letters.contains(&curr_index) && total_string == 4 {
                    total_string = 0;
                    curr_index += 1;
                }
            }
        }
    }

    let mut all_letters = VecDeque::new();

    for digit in digits.chars() {
        let digit = digit.to_digit(10).unwrap();

        if let Some(possible_letters) = dic.get(&digit) {
            all_letters.push_back(possible_letters.to_vec());
        }
    }

    while all_letters.len() > 1 {
        let first_letters = all_letters.pop_front().unwrap();
        let second_letters = all_letters.pop_front().unwrap();

        let mut compose = vec![];

        for first_string in &first_letters {
            for second_string in &second_letters {
                let mut the_first = first_string.clone();

                the_first.push_str(second_string.as_str());
                compose.push(the_first);
            }
        }

        all_letters.push_front(compose);
    }

    return all_letters[0].to_vec();
}

#[cfg(test)]
mod test_letter_comb_mod {
    use crate::recursion::letter_combinations::letter_combinations;
    #[test]
    fn test_letter_comb() {
        let letter_combo = letter_combinations("1234".to_string());

        assert_eq!(letter_combo.len(), 27);

        let letter_combo = letter_combinations("23".to_string());

        assert_eq!(letter_combo.len(), 9);

        let letter_combo = letter_combinations("2".to_string());

        assert_eq!(letter_combo.len(), 3);

        let letter_combo = letter_combinations("".to_string());

        assert_eq!(letter_combo.len(), 0);
    }
}
