const express = require('express');
const router = express.Router();

// Assuming we have a migrationUtil that handles database migrations
const migrationUtil = require('./migration_util');

router.post('/run-migrations', async (req, res) => {
    try {
        // Authenticate the request to ensure it's authorized
        if (!authenticateRequest(req)) {
            return res.status(401).send('Unauthorized');
        }

        const result = await migrationUtil.runMigrations();
        res.json({
            message: 'Migrations completed successfully',
            details: result
        });
    } catch (error) {
        console.error('Failed to complete migrations:', error);
        res.status(500).send('Failed to run migrations');
    }
});

function authenticateRequest(req) {
    // Implement your authentication logic here
    // For example, check for a specific header or API key
    const apiKey = req.headers['x-api-key'];
    return apiKey === process.env.UPGRADE_API_KEY;
}

module.exports = router;