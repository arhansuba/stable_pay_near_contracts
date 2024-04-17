// Assuming Express.js setup
const express = require('express');
const NearService = require('./services/NearService');
const app = express();
const PORT = process.env.PORT || 3000;

app.use(express.json());

const nearService = new NearService();

app.get('/api/data', async (req, res) => {
    try {
        const parameter = req.query.parameter;
        const data = await nearService.getContractData(parameter);
        res.json({ data });
    } catch (error) {
        console.error('Failed to fetch data:', error);
        res.status(500).send('Failed to fetch data from smart contract');
    }
});

app.post('/api/transaction', async (req, res) => {
    try {
        const { receiverId, amount } = req.body;
        const transaction = await nearService.createTransaction(receiverId, amount);
        res.json({ transaction });
    } catch (error) {
        console.error('Failed to create transaction:', error);
        res.status(500).send('Failed to send transaction to smart contract');
    }
});

app.listen(PORT, () => {
    console.log(`Server running on http://localhost:${PORT}`);
});