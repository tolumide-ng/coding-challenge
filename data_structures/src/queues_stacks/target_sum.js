function getTargetSum(target, numbers) {
    let count = 0;

    function getTotal(sum, index) {
        if (index == numbers.length) {
            if (sum == target) {
                count += 1;
            }
        } else {
            getTotal(sum - numbers[index], index + 1);
            getTotal(sum + numbers[index], index + 1);
        }
    }

    getTotal(0, 0);

    return count;
}
