// src/controllers/AuthController.js
const jwt = require('jsonwebtoken');
const { connect, keyStores } = require('near-api-js');
const nearConfig = require('../config/nearConfig');

// Utility function to initialize NEAR connection
async function initNear() {
    const keyStore = new keyStores.InMemoryKeyStore();
    const near = await connect({ ...nearConfig, keyStore });
    return near;
}

exports.login = async (req, res) => {
    const { accountId } = req.body;

    if (!accountId) {
        return res.status(400).json({ error: "AccountId is required" });
    }

    try {
        const near = await initNear();
        const account = await near.account(accountId);

        // Perform any additional verification if needed
        // For example, check if the accountId is registered in your app's database

        // Generate a JWT token (or any other method you use for session management)
        const token = jwt.sign(
            { accountId: account.accountId },
            process.env.JWT_SECRET,
            { expiresIn: '1h' } // Token expires in 1 hour
        );

        return res.json({ message: "Authentication successful", token });
    } catch (error) {
        console.error("Login error:", error);
        return res.status(500).json({ error: "Authentication failed" });
    }
};