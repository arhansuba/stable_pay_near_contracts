// controllers/RestAPIController.js
const express = require('express');
const router = express.Router();
const blockchainService = require('../services/BlockchainInteractionService');
const { validateTransferInput } = require('../middleware/validationMiddleware');
const { authenticateUser } = require('../middleware/authenticationMiddleware');

// Endpoint to get the balance of a user's stablecoin account
router.get('/stablecoin/balance/:accountId', authenticateUser, async (req, res) => {
    try {
        const { accountId } = req.params;
        const balance = await blockchainService.getStablecoinBalance(accountId);
        res.json({ balance });
    } catch (error) {
        console.error('Error fetching stablecoin balance:', error);
        res.status(500).send('Failed to fetch stablecoin balance');
    }
});

// Endpoint to transfer stablecoins from one user to another
router.post('/stablecoin/transfer', authenticateUser, validateTransferInput, async (req, res) => {
    try {
        const { senderId, receiverId, amount } = req.body;
        
        // Perform transfer
        const transferResult = await blockchainService.transferStablecoins({ senderId, receiverId, amount });
        
        res.json({ transferResult });
    } catch (error) {
        console.error('Error transferring stablecoins:', error);
        res.status(500).send('Failed to transfer stablecoins');
    }
});

// Assuming express setup
router.get('/usdc/balance/:accountId', async (req, res) => {
    try {
      const balance = await getUSDCBalance(req.params.accountId, process.env.NODE_ENV !== 'production');
      res.json({ balance: balance });
    } catch (error) {
      console.error('Failed to get USDC balance:', error);
      res.status(500).json({ error: 'Failed to retrieve balance' });
    }
  });

  // RestAPIController.js
router.post('/send-payment', async (req, res) => {
    // Extract details from req.body and call your service method
});

router.get('/balance/:accountId', async (req, res) => {
    // Call your service method to get balance and return it
});

// Additional endpoints for additional functionalities...

module.exports = router;
