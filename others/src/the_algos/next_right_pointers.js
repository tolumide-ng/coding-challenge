/**
 * // Definition for a Node.
 * function Node(val, left, right, next) {
 *    this.val = val === undefined ? null : val;
 *    this.left = left === undefined ? null : left;
 *    this.right = right === undefined ? null : right;
 *    this.next = next === undefined ? null : next;
 * };
 */

/**
 * @param {Node} root
 * @return {Node}
 */

const connect = function (root) {
    let startNode = root;

    while (startNode !== null) {
        let curr = startNode;
        while (curr !== null) {
            if (curr.left != null) {
                curr.left.next = curr.right;
            }
            if (curr.right !== null && curr.next !== null) {
                curr.right.next = curr.next.left;
            }
            curr = curr.next;
        }
        startNode = startNode.left;
    }
    return root;
};
