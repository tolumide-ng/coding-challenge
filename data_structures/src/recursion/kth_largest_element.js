function kthLargest(nums, k) {
    return nums.sort((a, b) => b - a)[k - 1];
}
