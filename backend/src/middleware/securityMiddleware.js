const helmet = require('helmet');
const xssClean = require('xss-clean');
const hpp = require('hpp');
const csurf = require('csurf');
const rateLimitingConfig = require('../config/rateLimitingConfig');

// Apply security headers with Helmet
function setSecurityHeaders(app) {
    app.use(helmet());
    // Customize helmet as needed, e.g., contentSecurityPolicy for defining allowed sources
}

// Protect against XSS attacks
function preventXSS(app) {
    app.use(xssClean());
}

// Prevent HTTP Parameter Pollution
function preventHPP(app) {
    app.use(hpp());
}

// CSRF protection
function csrfProtection(app) {
    app.use(csurf({ cookie: true }));
}

// Apply rate limiting to all routes and specific ones for stablecoin transactions
function applyRateLimiting(app) {
    app.use(rateLimitingConfig.generalLimiter);
    // Apply stablecoinTransactionLimiter specifically on stablecoin transaction routes
    // Example: app.use('/api/stablecoin', rateLimitingConfig.stablecoinTransactionLimiter);
}

module.exports = {
    setSecurityHeaders,
    preventXSS,
    preventHPP,
    csrfProtection,
    applyRateLimiting,
};