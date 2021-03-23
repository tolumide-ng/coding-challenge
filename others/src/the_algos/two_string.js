function twoStrings(s1, s2) {
    let possible = "NO";

    for (let i = 0; i < s1.length; i++) {
        if (s2.includes(s1[i])) {
            possible = "YES";
        }
    }

    return possible;
}
