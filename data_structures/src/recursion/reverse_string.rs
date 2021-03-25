pub fn reverse_string(s: &mut Vec<char>) {
    if s.is_empty() {
        return;
    }

    let mut target: usize = 0;
    let mut current_index = s.len() - 1;
    let mut reader = (((s.len() - 1) / 2) as f32).floor() as i32;

    reverse_helper(s, &mut target, &mut current_index, &mut reader);
}

pub fn reverse_helper(
    s: &mut Vec<char>,
    target: &mut usize,
    current_index: &mut usize,
    reader: &mut i32,
) {
    if *reader < 0 || *target == *current_index {
        return;
    }

    let target_char = s[*target];
    let current_index_char = s[*current_index];

    s[*target] = current_index_char;
    s[*current_index] = target_char;

    *target += 1;
    *current_index -= 1;
    *reader -= 1;

    reverse_helper(s, target, current_index, reader);
}

pub fn non_recursive(s: &mut Vec<char>) {
    let mut l = 0;
    let mut r = s.len() - 1;

    while l < r {
        s.swap(l, r);
        l += 1;
        r -= 1;
    }
}

pub fn generate_vchars(s: Vec<&str>) -> Vec<char> {
    let mut d = String::from("");

    for x in s {
        d.push_str(x);
    }

    let v: Vec<char> = d.chars().collect();

    return v;
}

#[cfg(test)]
mod test_reverse_string_cont {
    use super::*;

    #[test]
    fn test_reverse_string() {
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];

        reverse_string(&mut s);

        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);

        let mut pred = vec![
            "A", " ", "m", "a", "n", ",", " ", "a", " ", "p", "l", "a", "n", ",", " ", "a", " ",
            "c", "a", "n", "a", "l", ":", " ", "P", "a", "n", "a", "m", "a",
        ];

        let mut s = generate_vchars(pred);

        reverse_string(&mut s);

        assert_eq!(
            s,
            generate_vchars(vec![
                "a", "m", "a", "n", "a", "P", " ", ":", "l", "a", "n", "a", "c", " ", "a", " ",
                ",", "n", "a", "l", "p", " ", "a", " ", ",", "n", "a", "m", " ", "A"
            ])
        );

        //     let hello: String = chars.iter()
        // .map(|&x| x as u8)
        // .map(|x| (x + 1) as char)
        // .collect();
    }
}
