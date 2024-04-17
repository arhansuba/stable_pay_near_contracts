const express = require('express');
const authController = require('./controllers/AuthController');
const paymentController = require('./controllers/PaymentController');
const userController = require('./controllers/UserController');
const rateLimitingConfig = require('./config/rateLimitingConfig');

function registerApiEndpoints(app) {
    const apiRouter = express.Router();

    // Apply rate limiting to all API routes
    apiRouter.use(rateLimitingConfig);

    // Auth routes
    apiRouter.use('/auth', authController);

    // Payment routes
    apiRouter.use('/payments', paymentController);

    // User management routes
    apiRouter.use('/users', userController);

    // Register the API router with the app
    app.use('/api', apiRouter);
}

module.exports = registerApiEndpoints;