const sortArray = function (nums) {
    if (nums.length <= 1) {
        return nums;
    }

    let store = [];

    let pivot = ~~(nums.length / 2);
    let left = sortArray(nums.slice(0, pivot));
    let right = sortArray(nums.slice(pivot));

    while (left.length > 0 && right.length > 0) {
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
