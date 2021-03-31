function getRow(rowIndex) {
    let theRowIndex = rowIndex + 1;
    let row = new Array(theRowIndex).fill(1);

    for (let i = 2; i < theRowIndex; i++) {
        for (let j = i; j > 1; j--) {
            row[j] += row[j - 1];
        }
    }

    return row;
}
