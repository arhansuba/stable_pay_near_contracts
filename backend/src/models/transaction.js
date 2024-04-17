// src/models/Transaction.js
const mongoose = require('mongoose');

const TransactionSchema = new mongoose.Schema({
    transactionHash: { type: String, required: true, unique: true },
    fromAccountId: { type: String, required: true },
    toAccountId: { type: String, required: false }, // Might not be required for certain types of transactions
    amount: { type: Number, required: false }, // Not all transactions will involve a transfer of value
    type: { type: String, required: true, enum: ['transfer', 'contractCall', 'stake', 'createAccount', 'others'] },
    status: { type: String, required: true, enum: ['success', 'failure'] },
    timestamp: { type: Date, default: Date.now }
});

module.exports = mongoose.model('Transaction', TransactionSchema);