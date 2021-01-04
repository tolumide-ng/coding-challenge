/// COUNTCONSTRUCT
/// Write a function `countConstruct(target, wordBank)` that accepts a target string and an array of strings
/// The function should return the number of ways that target can be constructed by concatenating words of the `wordbank` array
/// You may reuse elements of `wordBank` as many times as possible

pub mod count_construct {
    use std::collections::HashMap;

    struct CountConstruct {
        dic: Vec<&'static str>,
        memo: HashMap<String, usize>,
        // total_count: usize,
    }

    impl CountConstruct {
        pub fn new(dic: Vec<&'static str>) -> Self {
            return CountConstruct {
                dic,
                memo: HashMap::new(),
                // total_count: 0,
            };
        }
    }

    pub fn recursive_count_construct(target: &str, dic: Vec<&'static str>) -> usize {
        impl CountConstruct {
            pub fn get_construct_count(&self, target: &str) -> usize {
                if target.len() == 0 {
                    return 1;
                }

                let mut total_count = 0;

                for word in &self.dic {
                    if target.starts_with(word) {
                        let (_, new_target) = target.split_at(word.len());
                        let result = self.get_construct_count(new_target);

                        if result > 0 {
                            total_count += result;
                        }
                    }
                }

                return total_count;
            }
        }

        let result = CountConstruct::new(dic);
        return result.get_construct_count(target);
    }

    pub fn memoized_count_construct(target: &str, dic: Vec<&'static str>) -> usize {
        impl CountConstruct {
            pub fn get_memoized_count_construct(&mut self, target: &str) -> usize {
                if self.memo.contains_key(&target.to_owned()) {
                    let value = self.memo.get(target).unwrap();
                    return *value;
                }

                if target.len() == 0 {
                    return 1;
                }

                let mut total_count = 0;

                let all_dics = self.dic.clone();

                for word in all_dics {
                    if target.starts_with(word) {
                        let (_, new_target) = target.split_at(word.len());
                        let result = self.get_memoized_count_construct(new_target);
                        if result > 0 {
                            self.memo.insert(new_target.to_owned(), result);
                            total_count += result;
                        }
                    }
                }

                self.memo.insert(target.to_owned(), total_count);
                return total_count;
            }
        }

        let mut result = CountConstruct::new(dic);
        return result.get_memoized_count_construct(target);
    }

    pub fn tabulated_count_construct(target: &str, dic: Vec<&'static str>) -> usize {
        impl CountConstruct {
            pub fn get_tabulated_count_construct(&self, target: &str) -> usize {
                let mut vec_store = vec![0; target.len() + 1];

                // there is only one way to count no words in the dic(tionary)
                vec_store[0] = 1;

                for value in 0..=target.len() {
                    if vec_store[value] != 0 {
                        for word in &self.dic {
                            let word_len = word.len();
                            let expected_len = value + word_len;

                            if target.get(value..expected_len).is_some()
                                && expected_len <= target.len()
                            {
                                let obtained_word = target.get(value..expected_len).unwrap();

                                if obtained_word == *word {
                                    vec_store[expected_len] += vec_store[value];
                                }
                            }
                        }
                    }
                }

                return vec_store[target.len()];
            }
        }

        let result = CountConstruct::new(dic);
        return result.get_tabulated_count_construct(target);
    }
}

#[test]
fn recursive_count_construct() {
    use count_construct::recursive_count_construct;

    assert_eq!(
        recursive_count_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]),
        2
    );
    assert_eq!(
        recursive_count_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]),
        1
    );
    assert_eq!(
        recursive_count_construct(
            "enterapotentpot",
            vec!["a", "p", "ent", "enter", "ot", "o", "t"]
        ),
        4
    );
    assert_eq!(
        recursive_count_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]),
        1
    );
}

#[test]
fn memoized_count_construct() {
    use count_construct::memoized_count_construct;

    assert_eq!(
        memoized_count_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]),
        2
    );
    assert_eq!(
        memoized_count_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]),
        1
    );
    assert_eq!(
        memoized_count_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            vec!["e", "ee", "eee", "eeee", "eeeee", "eeeeee", "eeeeeee"]
        ),
        0
    );
}

#[test]
fn tabulated_count_construct() {
    use count_construct::tabulated_count_construct;

    assert_eq!(
        tabulated_count_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]),
        2
    );
    assert_eq!(
        tabulated_count_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]),
        1
    );
    assert_eq!(
        tabulated_count_construct(
            "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeef",
            vec!["e", "ee", "eee", "eeee", "eeeee", "eeeeee", "eeeeeee"]
        ),
        0
    );
    assert_eq!(
        tabulated_count_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]),
        2
    )
}
