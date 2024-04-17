// src/models/Block.js
const mongoose = require('mongoose');

const BlockSchema = new mongoose.Schema({
    blockHeight: { type: Number, required: true, unique: true },
    blockHash: { type: String, required: true, unique: true },
    prevBlockHash: { type: String, required: true },
    timestamp: { type: Date, required: true },
    transactions: [{ type: mongoose.Schema.Types.ObjectId, ref: 'Transaction' }] // References Transaction model
});

module.exports = mongoose.model('Block', BlockSchema);