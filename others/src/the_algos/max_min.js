function maxMin(k, arr) {
    if (k > arr.length) {
        return 0;
    }
    arr.sort((a, b) => a - b);

    let result = 10000000000;

    for (let i = 0; i + k - 1 < arr.length; i++) {
        result = Math.min(result, arr[i + k - 1] - arr[i]);
    }

    return result;
}

console.assert(
    maxMin(5, [
        4504,
        1520,
        5857,
        4094,
        4157,
        3902,
        822,
        6643,
        2422,
        7288,
        8245,
        9948,
        2822,
        1784,
        7802,
        3142,
        9739,
        5629,
        5413,
        7232,
    ]),
    5
);
