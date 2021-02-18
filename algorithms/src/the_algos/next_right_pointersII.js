const connect = function (root) {
    let queue = [];

    queue.push(root);

    while (queue.length) {
        // let curr = queue.shift(1);
        const queueLength = queue.length;

        for (let i = 0; i < queueLength; i++) {
            const currValue = queue.shift(0);
            if (currValue.left) {
                queue.push(currValue.left);
            }

            if (currValue.right) {
                queue.push(currValue.right);
            }

            if (i !== queueLength - 1) {
                currValue.next = queue[0];
            }

            if (i === queueLength - 1) {
                currValue.next = null;
            }
        }
    }

    return root;
};
