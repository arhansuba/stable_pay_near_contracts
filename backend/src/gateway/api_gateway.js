// api_gateway.js
const express = require('express');
const httpProxy = require('express-http-proxy');
const { apiLimiter, apiKeyValidator } = require('./security_enhancements');

const userServiceProxy = httpProxy('http://localhost:3001');
const paymentServiceProxy = httpProxy('http://localhost:3002');

const app = express();

// Middleware for logging
app.use((req, res, next) => {
  console.log(`Received request: ${req.method} ${req.originalUrl}`);
  next();
});

// Global rate limiter
app.use(apiLimiter);

// API Key validation for all routes
app.use(apiKeyValidator);

// Route to User Service
app.use('/users', (req, res, next) => {
  userServiceProxy(req, res, next);
});

// Route to Payment Service
app.use('/payments', (req, res, next) => {
  paymentServiceProxy(req, res, next);
});

// You can add more routes and services as needed

const PORT = process.env.PORT || 3000;
app.listen(PORT, () => console.log(`API Gateway running on port ${PORT}`));