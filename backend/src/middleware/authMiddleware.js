// src/middleware/authMiddleware.js
const jwt = require('jsonwebtoken');

const authenticateToken = (req, res, next) => {
    // Typically, the token is sent in the "Authorization" header
    const authHeader = req.headers['authorization'];
    const token = authHeader && authHeader.split(' ')[1]; // "Bearer TOKEN"

    if (!token) {
        return res.status(401).json({ error: 'A token is required for authentication' });
    }

    try {
        // Verify the token using the same secret used to sign the JWT
        const decoded = jwt.verify(token, process.env.JWT_SECRET);
        // Attach the decoded user (payload) to the request to use in your routes
        req.user = decoded;
        next(); // Proceed to the next middleware/function in the stack
    } catch (error) {
        return res.status(403).json({ error: 'Invalid token' });
    }
};

module.exports = authenticateToken;