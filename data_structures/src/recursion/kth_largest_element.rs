pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut cloned_nums = nums.clone();
    cloned_nums.sort_by(|a, b| b.cmp(a));

    return cloned_nums[(k - 1) as usize];
}

#[cfg(test)]
mod test_find_kth_largest_mod {
    use crate::recursion::kth_largest_element::find_kth_largest;

    #[test]
    fn test_find_kth_largest() {
        assert_eq!(find_kth_largest(vec![3, 5, 1, 2, 9, 19, -45], 3), 5);
        assert_eq!(find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4), 4);
        assert_eq!(find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2), 5);
    }
}
