/**
 * @param {number} x
 * @param {number} n
 * @return {number}
 */
var myPow = function (x, n) {
    if (n == 0) {
        return 1;
    }

    if (n == 1) {
        return x;
    }

    if (n < 0) {
        return myPow(1 / x, -n);
    }

    let result = myPow(x * x, Math.floor(n / 2));

    if (n % 2) {
        result *= x;
        return result;
    }

    return result;
};
