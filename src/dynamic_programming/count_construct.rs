/// COUNTCONSTRUCT
/// Write a function `countConstruct(target, wordBank)` that accepts a target string and an array of strings
/// The function should return the number of ways that target can be constructed by concatenating words of the `wordbank` array
/// You may reuse elements of `wordBank` as many times as possible

pub mod count_construct {
    use std::collections::HashMap;

    struct CountConstruct {
        dic: Vec<&'static str>,
        memo: HashMap<String, bool>,
        total_count: usize,
    }

    impl CountConstruct {
        pub fn new(dic: Vec<&'static str>) -> Self {
            return CountConstruct {
                dic,
                memo: HashMap::new(),
                total_count: 0,
            };
        }
    }

    pub fn recursive_count_construct(target: &str, dic: Vec<&'static str>) -> usize {
        impl CountConstruct {
            pub fn get_construct_count(&mut self, target: &str) -> usize {
                if target.len() == 0 {
                    return 1;
                }

                let all_dics = &self.dic.clone();

                for word in all_dics {
                    if target.starts_with(word) {
                        let (_, new_target) = target.split_at(word.len());
                        let result = self.get_construct_count(new_target);

                        if result > 0 {
                            self.total_count += 1;

                            return self.total_count;
                        }
                    }
                }

                return 0;
            }
        }

        let mut result = CountConstruct::new(dic);
        return result.get_construct_count(target);
    }
}

#[test]
fn recursive_count_construct() {
    use count_construct::recursive_count_construct;

    assert_eq!(
        recursive_count_construct("purple", vec!["purp", "p", "ur", "le", "purpl"]),
        2
    )
}
