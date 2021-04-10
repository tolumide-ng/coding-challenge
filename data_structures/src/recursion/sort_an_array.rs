pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    let mut store = vec![];

    if nums.len() <= 1 {
        return nums;
    }

    let pivot = (nums.len() / 2) as usize;
    let mut left = sort_array(nums[0..pivot].to_vec());
    let mut right = sort_array(nums[pivot..].to_vec());

    while left.len() > 0 && right.len() > 0 {
        if left[0] < right[0] {
            store.push(left[0]);
            left.remove(0);
        } else {
            store.push(right[0]);
            right.remove(0);
        }
    }

    store.append(&mut left);
    store.append(&mut right);

    return store;
}

#[cfg(test)]
mod test_sort_array_mod {
    use crate::recursion::sort_an_array::sort_array;

    #[test]
    fn test_sort_array() {
        assert_eq!(sort_array(vec![]), vec![]);
        assert_eq!(sort_array(vec![3, 5, 6, 1, 10, 2]), vec![1, 2, 3, 5, 6, 10]);
    }
}
