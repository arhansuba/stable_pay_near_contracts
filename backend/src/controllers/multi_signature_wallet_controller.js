const express = require('express');
const router = express.Router();

// Placeholder for a service that handles the blockchain interactions
const multiSigWalletService = require('./multi_sig_wallet_service');

router.post('/create-transaction', async (req, res) => {
    try {
        const { to, value, data } = req.body;
        const transactionId = await multiSigWalletService.createTransaction(to, value, data);
        res.json({ message: 'Transaction proposed successfully.', transactionId });
    } catch (error) {
        console.error('Error creating transaction:', error);
        res.status(500).send('Failed to propose transaction');
    }
});

router.post('/submit-signature', async (req, res) => {
    try {
        const { transactionId, signature } = req.body;
        await multiSigWalletService.submitSignature(transactionId, signature);
        res.json({ message: 'Signature submitted successfully.' });
    } catch (error) {
        console.error('Error submitting signature:', error);
        res.status(500).send('Failed to submit signature');
    }
});

router.post('/execute-transaction', async (req, res) => {
    try {
        const { transactionId } = req.body;
        await multiSigWalletService.executeTransaction(transactionId);
        res.json({ message: 'Transaction executed successfully.' });
    } catch (error) {
        console.error('Error executing transaction:', error);
        res.status(500).send('Failed to execute transaction');
    }
});

module.exports = router;