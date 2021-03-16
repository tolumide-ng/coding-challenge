/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {number[]} inorder
 * @param {number[]} postorder
 * @return {TreeNode}
 */

function TreeNode(val, left, right) {
    this.val = val === undefined ? 0 : val;
    this.left = left === undefined ? null : left;
    this.right = right === undefined ? null : right;
}

const buildTree = function (inorder, postorder) {
    return recursive_build_tree({
        inorder,
        postorder,
    });
};

function recursive_build_tree({ inorder, postorder }) {
    if (inorder.length === 0) {
        return null;
    }

    let root_index = null;
    let root = null;

    let postorder_length = postorder.length;

    for (let i = postorder_length; i--; i >= 0) {
        if (inorder.includes(postorder[i]) && root_index === null) {
            root_index = i;
            root = postorder[i];
            // return;
        }
    }

    let tree = {
        val: postorder[root_index],
    };

    root_index = postorder.length - 1;

    let inorder_index = inorder.indexOf(root);

    let left = inorder.slice(0, inorder_index);
    let right = inorder.slice(inorder_index + 1);

    let the_left = recursive_build_tree({ inorder: left, postorder });
    let the_right = recursive_build_tree({ inorder: right, postorder });

    tree.left = the_left;
    tree.right = the_right;

    return tree;
}
