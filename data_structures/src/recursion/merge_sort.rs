pub mod merge_sort {
    pub fn top_down(nums: Vec<i32>) -> Vec<i32> {
        let mut store: Vec<i32> = vec![];

        if nums.len() <= 1 {
            return nums;
        }

        let pivot = nums.len() / 2;

        let mut left = vec![0; pivot];
        left.copy_from_slice(&nums[0..pivot]);

        let mut left = top_down(left);

        let mut right = vec![0; nums.len() - pivot];
        right.copy_from_slice(&nums[pivot..]);
        let mut right = top_down(right);

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
}

#[cfg(test)]
mod test_merge_sort_mod {
    use crate::recursion::merge_sort::merge_sort::top_down;

    #[test]
    fn test_top_down_approach() {
        assert_eq!(top_down(vec![4, 5, 2, 1, 3]), vec![1, 2, 3, 4, 5]);
        assert_eq!(top_down(vec![7, 5, 99, 10]), vec![5, 7, 10, 99]);
    }
}
