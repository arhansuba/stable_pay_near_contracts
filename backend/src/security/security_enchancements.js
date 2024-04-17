// security_enhancements.js

const rateLimit = require('express-rate-limit');
const helmet = require('helmet');
const cors = require('cors');

// Apply basic security policies with Helmet
function applySecurityPolicies(app) {
  app.use(helmet());
}

// Enable CORS with various options
function setupCors(app) {
  app.use(cors({
    // Configure based on your specific needs
    origin: '*', // Be more restrictive depending on your application's requirements
  }));
}

// Rate limiting to prevent abuse
const apiLimiter = rateLimit({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 100, // Limit each IP to 100 requests per windowMs
  standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
  legacyHeaders: false, // Disable the `X-RateLimit-*` headers
  message: 'Too many requests from this IP, please try again after 15 minutes',
});

// API key validation middleware for sensitive routes
function apiKeyValidator(req, res, next) {
  const apiKey = req.header('X-API-Key');
  // Replace 'your_api_key_here' with your actual API key value or use an environment variable
  if (!apiKey || apiKey !== process.env.API_KEY) {
    return res.status(401).json({ error: 'Unauthorized: Invalid or missing API key' });
  }
  next();
}

module.exports = {
  applySecurityPolicies,
  setupCors,
  apiLimiter,
  apiKeyValidator,
};