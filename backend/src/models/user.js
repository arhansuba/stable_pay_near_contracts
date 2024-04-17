const mongoose = require('mongoose');

const UserSchema = new mongoose.Schema({
    accountId: { type: String, required: true, unique: true }, // NEAR account ID
    name: { type: String, required: false },
    email: { type: String, required: false },
    // Add other fields as needed
}, { timestamps: true });

module.exports = mongoose.model('User', UserSchema);