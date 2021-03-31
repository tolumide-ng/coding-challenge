function climbStairs(n) {
    if (n < 2) {
        return n;
    }

    let times = [1, 2];

    for (let i = 2; i < n; i++) {
        let newTimes = times[0] + times[1];
        times[0] = times[1];
        times[1] = newTimes;
    }

    return times[1];
}
