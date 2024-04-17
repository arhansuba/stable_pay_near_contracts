const express = require('express');
const router = express.Router();

// Assuming a service that handles fetching analytics data
const analyticsService = require('./analyticsService');

// Route for fetching general analytics overview
router.get('/overview', async (req, res) => {
    try {
        const overviewData = await analyticsService.getOverviewData();
        res.json(overviewData);
    } catch (error) {
        console.error('Error fetching analytics overview:', error);
        res.status(500).send('Failed to fetch analytics overview');
    }
});

// Route for fetching detailed user engagement analytics
router.get('/user-engagement', async (req, res) => {
    try {
        const startDate = req.query.startDate;
        const endDate = req.query.endDate;
        const userEngagementData = await analyticsService.getUserEngagementData(startDate, endDate);
        res.json(userEngagementData);
    } catch (error) {
        console.error('Error fetching user engagement analytics:', error);
        res.status(500).send('Failed to fetch user engagement analytics');
    }
});

// Route for fetching transaction analytics
router.get('/transaction-analytics', async (req, res) => {
    try {
        const transactionAnalytics = await analyticsService.getTransactionAnalytics();
        res.json(transactionAnalytics);
    } catch (error) {
        console.error('Error fetching transaction analytics:', error);
        res.status(500).send('Failed to fetch transaction analytics');
    }
});

module.exports = router;