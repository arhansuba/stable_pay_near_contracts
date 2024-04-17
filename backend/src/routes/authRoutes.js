// src/routes/authRoutes.js

// Import the Express module
const express = require('express');

// Create a router instance
const router = express.Router();

// Import the AuthController and authService modules
const AuthController = require('../controllers/AuthController');
const authService = require('../services/authService');
const authenticateToken = require('../middleware/authMiddleware');

router.get('/profile', authenticateToken, UserController.getUserProfile);
// Define the '/login' route with a POST method
router.post('/login', AuthController.login);

// Export the router to make it available for use in other parts of the application
module.exports = router;
