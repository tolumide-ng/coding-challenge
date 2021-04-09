const mergeSort = (nums) => {
    let store = [];

    let pivot = ~~(nums.length / 2);
    let left = mergeSort(nums.slice(0, pivot));
    let right = mergeSort(nums.slice(pivot));

    while (left.length && right.length) {
        if (left[0] < right[0]) {
            store.push(left[0]);
            left.splice(0, 1);
        } else {
            store.push(right[0]);
            right.splice(0, 1);
        }
    }

    store = [...store, ...right, ...left];

    return store;
};
