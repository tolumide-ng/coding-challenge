/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {boolean}
 */
const isValidBST = function (root) {
    const CONSTRAINTMIN = -2147483649;
    let min = CONSTRAINTMIN;

    function confirmValidity(theRoot) {
        if (!theRoot) {
            return true;
        }

        if (!confirmValidity(theRoot.left)) {
            return false;
        }

        if (theRoot.val <= min) {
            return false;
        }

        min = theRoot.val;
        return confirmValidity(theRoot.right);
    }

    return confirmValidity(root);
};
