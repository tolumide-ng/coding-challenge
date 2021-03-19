class InorderTraversal {
    #output = [];

    constructor(root) {
        this.root = root;
    }

    #recursiveTraversal(root) {
        if (root !== null) {
            if (root?.left !== null) {
                this.recursiveTraversal(root.left);
            }

            this.#output.push(root.val);

            if (root?.right !== null) {
                this.recursiveTraversal(root.right);
            }
        }
    }

    get useRecursion() {
        this.#recursiveTraversal(root);
        return this.#output;
    }
}
