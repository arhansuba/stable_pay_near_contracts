const express = require('express');
const router = express.Router();

// Placeholder for the service that handles the logic
const multiSigWalletService = require('./multiSigWalletService');

// Route to create a new transaction that will require multiple signatures
router.post('/transactions/new', async (req, res) => {
    try {
        const { to, amount, data } = req.body;
        const transaction = await multiSigWalletService.createTransaction(to, amount, data);
        res.status(201).json(transaction);
    } catch (error) {
        console.error('Error creating new transaction:', error);
        res.status(500).send('Failed to create new transaction');
    }
});

// Route to submit a signature for a transaction
router.post('/transactions/:id/sign', async (req, res) => {
    try {
        const { signature } = req.body;
        const { id } = req.params;
        const result = await multiSigWalletService.submitSignature(id, signature);
        res.json(result);
    } catch (error) {
        console.error('Error submitting signature:', error);
        res.status(500).send('Failed to submit signature');
    }
});

// Route to execute a transaction after collecting required signatures
router.post('/transactions/:id/execute', async (req, res) => {
    try {
        const { id } = req.params;
        const result = await multiSigWalletService.executeTransaction(id);
        res.json(result);
    } catch (error) {
        console.error('Error executing transaction:', error);
        res.status(500).send('Failed to execute transaction');
    }
});

module.exports = router;