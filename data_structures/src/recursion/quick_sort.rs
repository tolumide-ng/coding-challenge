pub fn quick_sort(nums: &mut [i32]) {
    if nums.len() <= 1 {
        return;
    }

    let pivot: i32 = nums[nums.len() / 2];

    let mut left_marker: usize = 0;
    let mut right_marker: usize = nums.len() - 1;

    while left_marker < right_marker {
        while nums[left_marker] < pivot {
            left_marker += 1;
        }

        while nums[right_marker] > pivot {
            right_marker -= 1;
        }

        if left_marker <= right_marker {
            nums.swap(left_marker, right_marker);
            left_marker += 1;
            right_marker -= 1;
        }
    }

    quick_sort(&mut nums[..=right_marker]);
    quick_sort(&mut nums[left_marker..]);
}

#[test]
fn test_quick_sort() {
    use crate::recursion::quick_sort::quick_sort;

    let mut arr = vec![3, 5, 6, 1, 10];
    quick_sort(&mut arr);
    assert_eq!(arr, vec![1, 3, 5, 6, 10]);
}
