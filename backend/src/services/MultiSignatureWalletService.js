class MultiSignatureWalletService {
    constructor() {
        this.transactions = new Map(); // Holds transaction details
        this.requiredSignatures = 2; // Minimum required signatures
    }

    // Propose a new transaction
    proposeTransaction(id, transactionDetails) {
        if (this.transactions.has(id)) {
            throw new Error('Transaction ID already exists');
        }
        this.transactions.set(id, {
            ...transactionDetails,
            signatures: new Set(), // Unique collection of signatures
            executed: false,
        });
        return `Transaction ${id} proposed`;
    }

    // Submit a signature for a transaction
    submitSignature(transactionId, signature) {
        if (!this.transactions.has(transactionId)) {
            throw new Error('Transaction does not exist');
        }
        const transaction = this.transactions.get(transactionId);
        if (transaction.executed) {
            throw new Error('Transaction already executed');
        }
        transaction.signatures.add(signature);
        return `Signature added to transaction ${transactionId}`;
    }

    // Check if a transaction has enough signatures and execute it
    executeTransaction(transactionId) {
        if (!this.transactions.has(transactionId)) {
            throw new Error('Transaction does not exist');
        }
        const transaction = this.transactions.get(transactionId);
        if (transaction.executed) {
            throw new Error('Transaction already executed');
        }
        if (transaction.signatures.size < this.requiredSignatures) {
            throw new Error('Not enough signatures to execute the transaction');
        }
        transaction.executed = true;
        // Here, you would include the logic to execute the transaction on the blockchain
        return `Transaction ${transactionId} executed`;
    }
}

module.exports = MultiSignatureWalletService;