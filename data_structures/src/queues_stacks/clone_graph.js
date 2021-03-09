/**
 * // Definition for a Node.
 * function Node(val, neighbors) {
 *    this.val = val === undefined ? 0 : val;
 *    this.neighbors = neighbors === undefined ? [] : neighbors;
 * };
 */

/**
 * @param {Node} node
 * @return {Node}
 */
function cloneGraph(node) {
    process.stdout.write("\n\twhen>>>>>> ", node);
    const map = new Map();
    return clone(node);

    function clone(node) {
        if (!node) return null;
        if (map.has(node.val)) return map.get(node.val);

        const newNode = { val: node.val, neighbors: [] };
        map.set(node.val, newNode);
        for (let n of node.neighbors) {
            newNode.neighbors.push(clone(n));
        }
        // process.stdout.write("\nHERE>>>>>> {}", newNode);
        return newNode;
    }
}

let theNode = {
    val: 1,
    neighbors: [
        {
            val: 2,
            neighbors: [
                {
                    val: 3,
                    neighbors: [
                        { val: 5, neighbors: [] },
                        { val: 10, neighbors: [] },
                        {
                            val: 11,
                            neighbors: [{ val: 12, neighbors: [] }],
                        },
                    ],
                },
            ],
        },
        {
            val: 4,
            neighbors: [
                {
                    val: 6,
                    neighbors: [
                        {
                            val: 7,
                            neighbors: [
                                {
                                    val: 8,
                                    neighbors: [{ val: 9, neighbors: [] }],
                                },
                            ],
                        },
                    ],
                },
            ],
        },
    ],
};

cloneGraph({
    val: 3,
    neighbors: [
        { val: 5, neighbors: [] },
        { val: 10, neighbors: [] },
        {
            val: 11,
            neighbors: [{ val: 12, neighbors: [] }],
        },
    ],
});
