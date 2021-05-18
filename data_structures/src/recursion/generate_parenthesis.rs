struct Parenthesis {}
impl Parenthesis {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut list = vec![];

        Parenthesis::backtrack(&mut list, &mut String::from(""), n as usize, 0, 0);

        return list;
    }

    fn backtrack(
        list: &mut Vec<String>,
        current: &mut String,
        max: usize,
        open: usize,
        close: usize,
    ) {
        if current.len() == max * 2 {
            list.push(current.to_string());
            return;
        }

        if open < max {
            current.push_str("(");

            Parenthesis::backtrack(list, current, max, open + 1, close);
            current.pop();
        }

        if close < open {
            current.push_str(")");

            Parenthesis::backtrack(list, current, max, open, close + 1);
            current.pop();
        }
    }
}

#[cfg(test)]
mod test_generate_parenthesis_mod {
    use crate::recursion::generate_parenthesis::Parenthesis;

    #[test]
    fn test_generate_parenthesis() {
        let list = Parenthesis::generate_parenthesis(3);

        println!("the list {:#?}", list);

        assert_eq!(list.len(), 5);
    }
}
