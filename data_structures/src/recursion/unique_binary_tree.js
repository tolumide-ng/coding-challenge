/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {number} n
 * @return {TreeNode[]}
 */

const generateTree = (val, left, right) => ({
    val: val === undefined ? 0 : val,
    left: left === undefined ? null : left,
    right: right === undefined ? null : right,
});

const generateTrees = function (n) {
    return genHelper(1, n);
};

const genHelper = (start, end) => {
    const allTrees = [];

    if (start > end) {
        allTrees.push(undefined);
        return allTrees;
    }

    for (let i = start; i <= end; i++) {
        let left = genHelper(start, i - 1);
        let right = genHelper(i + 1, end);

        for (let left_index = 0; left_index < left.length; left_index++) {
            for (
                let right_index = 0;
                right_index < right.length;
                right_index++
            ) {
                let tree = generateTree(
                    i,
                    left[left_index],
                    right[right_index]
                );

                allTrees.push(tree);
            }
        }
    }

    return allTrees;
};
