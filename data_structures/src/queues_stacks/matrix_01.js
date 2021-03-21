const updateMatrix = (matrix) => {
    let dist = new Array(matrix.length);
  
  for(let i=0; i < matrix[0].length; i++){
    dist[i] = new Array(matrix.length).fill(Number.MAX_SAFE_INTEGER);
  }

    let queue = [];

    for (let x = 0; x < matrix.length; x++) {
        for (let y = 0; y < matrix[0].length; y++) {
            if (matrix[x][y] == 0) {
                queue.push({ x, y });
                dist[x][y] = 0;
            }
        }
    }

    const directions = [
        { x: 0, y: 1 },
        { x: 0, y: -1 },
        { x: 1, y: 0 },
        { x: -1, y: 0 },
    ];

    while (queue.length > 0) {
        let current = queue.shift();

        for (let position of directions) {
            let newX = current.x + position.x;
            let newY = current.y + position.y;

            if (
                newX < 0 ||
                newX >= matrix.length ||
                newY < 0 ||
                newY >= matrix[0].length
            ) {
                continue;
            }

            if (dist[newX][newY] > dist[current.x][current.y] + 1) {
                dist[newX][newY] = dist[current.x][current.y] + 1;
                queue.push({ x: newX, y: newY });
            }
        }
    }

    return dist;
};
