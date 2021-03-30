use std::collections::HashMap;

type SumType = Option<Vec<usize>>;

/// TIME = O(n) SPACE = 0(n)

pub fn two_sums(nums: Vec<usize>, target: usize) -> SumType {
    let mut result: SumType = None;
    let mut store: HashMap<usize, usize> = HashMap::new();

    for index in 0..nums.len() {
        let current_value = nums[index];

        if current_value <= target {
            let key = target - current_value;

            match store.get(&key) {
                Some(key) => {
                    result = Some(vec![*key, index]);
                    return result;
                }
                None => {
                    store.insert(current_value, index);
                }
            };
        }
    }

    result
}

pub fn two_sums_fast(nums: Vec<usize>, target: usize) -> SumType {
    let mut result: SumType = None;

    let mut sorted_list = nums.clone();

    sorted_list.sort_unstable();

    let mut lower_bound = 0;
    let mut upper_bound = sorted_list.len() - 1;

    let mut not_found = true;

    while lower_bound < upper_bound && not_found {
        if sorted_list[lower_bound] + sorted_list[upper_bound] == target {
            let first = nums
                .iter()
                .position(|&x| x == sorted_list[lower_bound])
                .unwrap();
            let second = nums
                .iter()
                .position(|&x| x == sorted_list[upper_bound])
                .unwrap();

            result = Some(vec![first, second]);
            not_found = false;
        }

        if sorted_list[lower_bound] + sorted_list[upper_bound] > target {
            upper_bound -= 1;
        }

        if sorted_list[lower_bound] + sorted_list[upper_bound] < target {
            lower_bound += 1;
        }
    }

    return result;
}

// #[cfg(test)]
// mod test_two_sums_mod {
//     #[test]
//     // fn two_sums_hash() {
//     //     use super::two_sums;
//     //     assert!(two_sums(vec![2, 5, 7, 11], 9).is_some());
//     //     assert!(two_sums(vec![2, 5, 9, 11], 9).is_none());
//     // }
//     #[test]
//     fn two_sums_fast_test() {
//         use super::two_sums_fast;
//         assert!(two_sums_fast(vec![2, 5, 7, 11], 9).is_some());
//         assert!(two_sums_fast(vec![2, 5, 9, 11], 9).is_none());
//     }
// }

#[cfg(test)]
mod test_two_sum_mod {
    #[test]
    fn test_two_sums_fast() {
        use super::two_sums_fast;
        assert!(two_sums_fast(vec![2, 5, 7, 11], 9).is_some());
        assert!(two_sums_fast(vec![2, 5, 9, 11], 9).is_none());
    }

    #[test]
    fn test_two_sums() {
        use super::two_sums;
        assert!(two_sums(vec![2, 5, 7, 11], 9).is_some());
        assert!(two_sums(vec![2, 5, 9, 11], 9).is_none());
    }
}
