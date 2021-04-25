function quickSort(list) {
    if (list.length <= 1) {
        return list;
    }

    let pivot = list[list.length - 1];

    const listLengthWithoutPivot = list.length - 2;
    let left = [];
    let right = [];

    for (let i = 0; i <= listLengthWithoutPivot; i++) {
        let current = list[i];

        if (current < pivot) {
            left.push(current);
        } else {
            right.push(current);
        }
    }

    let sorted_left = quickSort(left);
    let sorted_right = quickSort(right);

    return [...sorted_left, pivot, ...sorted_right];
}
