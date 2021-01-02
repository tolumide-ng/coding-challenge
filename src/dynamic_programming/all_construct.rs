/// Write a function `allConstruct(target, workBank)` that accepts a target string and an array of strings.
/// The function should return a 2D array containing all of the ways that the target can be constructed by concatenating
/// elements of the wordBank array. Each element of the 2d array should represeng one combiantion that constructs the target
/// You may resue elements of the wordBank as many times as you need

pub mod all_construct {
    use std::cell::RefCell;
    use std::collections::HashMap;

    type ReturnType = Vec<RefCell<Vec<&'static str>>>;

    #[derive(Debug)]
    struct AllConstructs {
        dic: Vec<&'static str>,
        memo: HashMap<String, &'static str>,
        aggregator: ReturnType,
        target: String,
    }

    impl AllConstructs {
        pub fn new(dic: Vec<&'static str>, target: String) -> Self {
            return AllConstructs {
                dic,
                memo: HashMap::new(),
                aggregator: Vec::new(),
                target,
            };
        }
    }

    pub fn recursive_all_construct(target: &'static str, dic: Vec<&'static str>) -> ReturnType {
        impl AllConstructs {
            pub fn get_all_recursive_constructs(&self, target: &'static str) -> ReturnType {
                if target.len() == 0 {
                    let d: RefCell<Vec<&str>> = RefCell::new(vec![]);
                    return vec![d];
                }

                let mut two_d_vec: ReturnType = vec![];

                for word in &self.dic {
                    if target.starts_with(word) {
                        let (_, new_target) = target.split_at(word.len());

                        let result = self.get_all_recursive_constructs(new_target);

                        for value in result.iter() {
                            *value.borrow_mut() =
                                [vec![*word], value.clone().into_inner()].concat();
                        }

                        two_d_vec = [two_d_vec, result].concat();
                    }
                }
                return two_d_vec;
            }
        }

        let result = AllConstructs::new(dic, target.to_owned());
        return result.get_all_recursive_constructs(target);
    }
}

#[test]
fn recsurive_all_constructs() {
    use all_construct::recursive_all_construct;

    assert_eq!(
        recursive_all_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd", "ef", "c"]).len(),
        4
    )
}


// 0 1 1 2 3 5 8