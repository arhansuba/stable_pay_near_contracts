const express = require('express');
const blockchainService = require('../services/blockchainService');
const BlockchainController = require('../controllers/BlockchainController');
const router = express.Router();

router.get('/price', BlockchainController.getCurrentNearPrice);
router.post('/send-tokens', BlockchainController.sendNearTokens);
router.post('/call-contract', BlockchainController.callSmartContractMethod);


// Route to call a smart contract method
router.post('/call-contract', async (req, res) => {
  const { methodName, args } = req.body;
  try {
    const result = await blockchainService.callContractMethod(methodName, args);
    res.json(result);
  } catch (error) {
    res.status(500).json({ error: error.message });
  }
});

module.exports = router;