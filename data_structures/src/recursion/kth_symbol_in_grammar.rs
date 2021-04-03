pub fn kth_grammar(n: i32, k: i32) -> i32 {
    kth_helper(n - 1, k, "0".to_string())
}

pub fn kth_helper(n: i32, k: i32, curr: String) -> i32 {
    if n == 0 && k <= curr.len() as i32 {
        let num = curr.as_bytes()[(k - 1) as usize];
        let num_char = num as char;
        let parsed_num_char: i32 = num_char.to_digit(10).unwrap() as i32;
        return parsed_num_char;
    }

    let mut new_curr = String::from("");

    for char in curr.chars() {
        match char.to_digit(10) {
            Some(digit) if digit == 0 => new_curr.push_str("01"),
            Some(digit) if digit == 1 => new_curr.push_str("10"),
            Some(_) => {}
            None => {}
        }
    }

    return kth_helper(n - 1, k, new_curr);
}

pub fn kth_grammar_fast(n: i32, k: i32) -> i32 {
    let mut s = u32::pow(2, (n - 1) as u32);
    let mut flips = 0;

    let mut k = k;

    while s > 2 {
        if k > (s / 2) as i32 {
            k -= (s / 2) as i32;
            flips += 1;
        }

        s /= 2;
    }

    k -= 1;
    if flips % 2 == 1 {
        k = 1 - k;
    }

    return k;
}

#[cfg(test)]
mod test_kth_grammar_mod {
    #[test]
    fn test_kth_grammar() {
        use crate::recursion::kth_symbol_in_grammar::kth_grammar;

        assert_eq!(kth_grammar(1, 1), 0);
        assert_eq!(kth_grammar(2, 1), 0);
        assert_eq!(kth_grammar(2, 2), 1);
        assert_eq!(kth_grammar(4, 5), 1);
        assert_eq!(kth_grammar(3, 3), 1);
    }
}
