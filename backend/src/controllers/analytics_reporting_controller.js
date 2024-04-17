const express = require('express');
const router = express.Router();
const analyticsService = require('./analyticsService');

// Endpoint for fetching summary analytics
router.get('/summary', async (req, res) => {
    try {
        const summaryData = await analyticsService.fetchSummaryAnalytics();
        res.json(summaryData);
    } catch (error) {
        console.error('Failed to fetch summary analytics:', error);
        res.status(500).send('Error fetching summary analytics');
    }
});

// Endpoint for fetching detailed transaction analytics
router.get('/transactions', async (req, res) => {
    try {
        const { startDate, endDate } = req.query;
        const transactionData = await analyticsService.fetchTransactionAnalytics(startDate, endDate);
        res.json(transactionData);
    } catch (error) {
        console.error('Failed to fetch transaction analytics:', error);
        res.status(500).send('Error fetching transaction analytics');
    }
});

module.exports = router;