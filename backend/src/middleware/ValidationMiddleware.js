// backend/ValidationMiddleware.js
exports.validateTransactionRequest = (req, res, next) => {
    const { senderId, receiverId, amount } = req.body;
    if (!senderId || !receiverId || !amount) {
        return res.status(400).json({ error: 'Missing required fields' });
    }
    // Add additional validation as necessary
    next();
};