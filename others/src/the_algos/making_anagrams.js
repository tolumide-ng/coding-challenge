function makingAnagrams(s1, s2) {
    const s1Dic = {};
    const s2Dic = {};

    s1.split("").map((letter) => {
        if (s1Dic[letter]) {
            s1Dic[letter] += 1;
        } else {
            s1Dic[letter] = 1;
        }
    });

    s2.split("").map((letter) => {
        if (s2Dic[letter]) {
            s2Dic[letter] += 1;
        } else {
            s2Dic[letter] = 1;
        }
    });

    let total = 0;

    Array.from(Object.keys(s1Dic)).map((letter) => {
        if (s2Dic[letter]) {
            total += Math.abs(s1Dic[letter] - s2Dic[letter]);
            s1Dic[letter] = 0;
            s2Dic[letter] = 0;
        }
    });

    Array.from(Object.keys(s1Dic)).map((letter) => {
        total += s1Dic[letter];
    });

    Array.from(Object.keys(s2Dic)).map((letter) => {
        total += s2Dic[letter];
    });

    return total;
}
