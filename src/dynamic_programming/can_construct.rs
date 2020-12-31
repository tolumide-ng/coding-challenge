/// CAN_CONSTRUCT
/// Write a function `canConstruct(target, wordBank)` that accepts a target string and
/// an array of strings.
/// The function should return a boolean indicating whether or not the `target` can be constructed
/// by concatenating elements of the wordBank array
/// You may resuse elements of workBank as many times as possible
///

pub mod can_construct {
    // type WordBankType = Vec<&'static str>;

    struct CanConstruct {
        dic: Vec<&'static str>,
    }

    impl CanConstruct {
        fn new(dic: Vec<&'static str>) -> CanConstruct {
            return CanConstruct { dic };
        }
    }

    pub fn recursive_can_construct(target: &str, dic: Vec<&'static str>) -> bool {
        impl CanConstruct {
            pub fn get_recursive_construct(&self, target: &str) -> bool {
                if target.len() == 0 {
                    return true;
                }

                for word in &self.dic {
                    if target.starts_with(word) {
                        let (_spliced_away, new_target) = target.split_at(word.len());
                        let result = self.get_recursive_construct(new_target);

                        if result {
                            return true;
                        }
                    }
                }

                return false;
            }
        }

        let result = CanConstruct::new(dic);
        return result.get_recursive_construct(target);
    }
}

#[test]
fn test_recursive_can_construct() {
    use can_construct::recursive_can_construct;

    assert_eq!(
        recursive_can_construct("tolumide", vec!["t", "ol", "umide"]),
        true
    );
    assert_eq!(
        recursive_can_construct("abcdef", vec!["ab", "abc", "cd", "def", "abcd"]),
        true
    );
    assert_eq!(
        recursive_can_construct(
            "skateboard",
            vec!["bo", "rd", "ate", "t", "ska", "sk", "boar"]
        ),
        false
    );
}
