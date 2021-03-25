function reverseString(s) {
    if (s.length == 0) {
        return;
    }

    let target = 0;
    let currentIndex = s.length - 1;
    let reader = Math.floor((s.length - 1) / 2);

    reverseHelper({ s, target, currentIndex, reader });

    function reverseHelper({ s, target, currentIndex, reader }) {
        if (reader === 0 || target === currentIndex) {
            return;
        }

        let targetChar = s[target];
        let currentChar = s[currentIndex];

        s[currentIndex] = targetChar;
        s[target] = currentChar;

        reverseHelper({
            s,
            target: target + 1,
            currentIndex: currentIndex - 1,
            reader: reader - 1,
        });
    }
}
