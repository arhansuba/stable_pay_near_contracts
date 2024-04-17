// backend/TransactionController.js
const express = require('express');
const router = express.Router();
const blockchainService = require('./BlockchainService');
const { validateTransactionRequest } = require('./ValidationMiddleware'); // Assume this middleware validates the request body

// Endpoint to initiate a transaction
router.post('/send', validateTransactionRequest, async (req, res) => {
    try {
        const { senderId, receiverId, amount } = req.body; // Ensure these values are validated
        const transactionHash = await blockchainService.sendPayment(senderId, receiverId, amount);
        return res.status(200).json({ message: 'Transaction initiated successfully', transactionHash });
    } catch (error) {
        console.error('Error sending transaction:', error);
        return res.status(500).json({ error: 'Failed to send transaction' });
    }
});

// Endpoint to check transaction status
router.get('/status/:transactionHash', async (req, res) => {
    try {
        const { transactionHash } = req.params;
        const status = await blockchainService.getTransactionStatus(transactionHash);
        return res.status(200).json({ transactionHash, status });
    } catch (error) {
        console.error('Error fetching transaction status:', error);
        return res.status(500).json({ error: 'Failed to fetch transaction status' });
    }
});

// Additional endpoint to fetch transaction history for an account
router.get('/history/:accountId', async (req, res) => {
    try {
        const { accountId } = req.params;
        const history = await blockchainService.getTransactionHistory(accountId);
        return res.status(200).json({ accountId, history });
    } catch (error) {
        console.error('Error fetching transaction history:', error);
        return res.status(500).json({ error: 'Failed to fetch transaction history' });
    }
});

// Endpoint to cancel a transaction (if possible based on blockchain logic)
router.post('/cancel', async (req, res) => {
    try {
        const { transactionHash } = req.body; // Assume validation
        const result = await blockchainService.cancelTransaction(transactionHash);
        return res.status(200).json({ message: 'Transaction cancellation initiated', result });
    } catch (error) {
        console.error('Error canceling transaction:', error);
        return res.status(500).json({ error: 'Failed to cancel transaction' });
    }
});

module.exports = router;