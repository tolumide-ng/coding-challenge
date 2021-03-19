class InorderTraversal {
    #output = [];
    #stack = [];

    constructor(root) {
        this.root = root;
    }

    get useRecursion() {
        this.#output = [];
        this.#stack = [];

        this.#recursiveTraversal(this.root);
        return this.#output;
    }

    get useIterative() {
        this.#output = [];
        this.#stack = [];

        this.#iterativetraversal(root);
        return this.#output;
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

    #iterativeTraversal(root) {
        let current = root;

        while (current !== null && this.#stack.length > 0) {
            while (current !== null) {
                this.#stack.push(current);
                current = current.left;
            }

            let value = stack.pop();
            this.#output.push(value);
            current = value.right;
        }
    }
}
