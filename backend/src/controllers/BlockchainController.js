// src/controllers/BlockchainController.js
const BlockchainService = require('../services/blockchainService/BlockchainService');

// Fetch the current NEAR token price
exports.getCurrentNearPrice = async (req, res) => {
    try {
        const price = await BlockchainService.getCurrentNearPrice();
        res.json({ price });
    } catch (error) {
        console.error('Error getting NEAR price:', error);
        res.status(500).json({ error: 'Failed to fetch NEAR price' });
    }
};

// Send NEAR tokens from one account to another
exports.sendNearTokens = async (req, res) => {
    const { senderId, receiverId, amount } = req.body;

    try {
        const transactionResult = await BlockchainService.sendNearTokens(senderId, receiverId, amount);
        res.json({ message: 'Transaction successful', transactionResult });
    } catch (error) {
        console.error('Error sending NEAR tokens:', error);
        res.status(500).json({ error: 'Failed to send NEAR tokens' });
    }
};

// Call a method on a smart contract
exports.callSmartContractMethod = async (req, res) => {
    const { contractId, methodName, args } = req.body;

    try {
        const result = await BlockchainService.callSmartContractMethod(contractId, methodName, args);
        res.json({ message: 'Smart contract method called successfully', result });
    } catch (error) {
        console.error('Error calling smart contract method:', error);
        res.status(500).json({ error: 'Failed to call smart contract method' });
    }
};