//www.hackerrank.com/challenges/game-of-thrones/problem?utm_campaign=challenge-recommendation&utm_medium=email&utm_source=24-hour-campaign

https: function gameOfThrones(s) {
    let numberOfOdds = 0;

    const wordsDic = {};

    for (let letter of s) {
        if (wordsDic[letter]) {
            wordsDic[letter] += 1;
        } else {
            wordDic[letter] = 1;
        }
    }

    Array.from(Object.keys(wordsDic)).map((letter) => {
        if (wordsDic[letter] % 2 !== 0) {
            numberOfOdds += 1;
        }
    });

    // Array.from(Object.keys(wordsDic)

    return [0, 1].includes(numberOfOdds) ? "YES" : "NO";
}
