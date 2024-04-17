// src/models/Payment.js
const mongoose = require('mongoose');

const PaymentSchema = new mongoose.Schema({
    senderAccountId: { type: String, required: true },
    receiverAccountId: { type: String, required: true },
    amount: { type: Number, required: true },
    currency: { type: String, default: 'NEAR' },
    status: { type: String, required: true, enum: ['pending', 'completed', 'failed'] },
    timestamp: { type: Date, default: Date.now }
});

module.exports = mongoose.model('Payment', PaymentSchema);