const express = require('express');
const router = express.Router();

// Assume we have an upgradeController that handles the logic
const upgradeController = require('./upgrade_controller');

// Secure middleware to check if the request is authorized
function checkAuth(req, res, next) {
    // Implement your authentication check here
    // This could involve checking a token, API key, or internal service header
    const apiKey = req.header('X-API-Key');
    if (apiKey === process.env.UPGRADE_API_KEY) {
        next();
    } else {
        return res.status(403).send('Unauthorized');
    }
}

// Route to trigger database migrations
router.post('/migrate-db', checkAuth, async (req, res) => {
    try {
        const result = await upgradeController.runDatabaseMigrations();
        res.json({ message: 'Database migration successful', details: result });
    } catch (error) {
        console.error('Database migration error:', error);
        res.status(500).send('Database migration failed');
    }
});

// Route to update application configurations
router.post('/update-config', checkAuth, async (req, res) => {
    try {
        const result = await upgradeController.updateConfigurations();
        res.json({ message: 'Configuration update successful', details: result });
    } catch (error) {
        console.error('Configuration update error:', error);
        res.status(500).send('Configuration update failed');
    }
});

module.exports = router;