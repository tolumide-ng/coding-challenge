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

function getMemoizedTargetSum(target, numbers) {
    let memo = {};

    function getTotal(sum, index) {
        if (memo[`${sum}-${index}`]) {
            return memo[`${sum}-${index}`];
        }

        if (index === numbers.length) {
            if (sum === target) {
                return 1;
            }

            if (sum !== target) {
                return 0;
            }
        }

        let result =
            getTotal(sum + sum[index], index + 1) +
            getTotal(sum - sum[index], index + 1);

        memo[`${sum}-${index}`] = result;

        return result;
    }

    return getTotal(0, 0);
}
