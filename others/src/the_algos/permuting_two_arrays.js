function twoArrays(k, A, B) {
    let possible = "YES";

    A.sort((a, b) => a - b);
    B.sort((a, b) => b - a);

    for (let i = 0; i < A.length; i++) {
        if (A[i] + B[i] < k) {
            possible = "NO";
        }
    }

    return possible;
}
