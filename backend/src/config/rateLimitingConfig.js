const rateLimit = require('express-rate-limit');
const RedisStore = require('rate-limit-redis');
const { createClient } = require('redis');

// Connect to Redis
const redisClient = createClient({ url: process.env.REDIS_URL });
redisClient.connect().catch(console.error);

// Define rate limiting configurations
const rateLimitConfig = {
    windowMs: 15 * 60 * 1000, // 15 minutes
    general: {
        maxRequests: 100, // limit each IP to 100 requests per window
        message: "Too many requests, please try again later."
    },
    stablecoinTransaction: {
        maxRequests: 20, // limit each IP to 20 requests per window for sensitive operations
        message: "Too many transaction requests, please try again later."
    }
};

// Create rate limit middleware
const createRateLimiter = (config) => {
    return rateLimit({
        store: new RedisStore({ client: redisClient }),
        windowMs: config.windowMs,
        max: config.maxRequests,
        message: config.message
    });
};

// General API rate limiting
const generalLimiter = createRateLimiter(rateLimitConfig.general);

// Stricter rate limiting for stablecoin transaction endpoints
const stablecoinTransactionLimiter = createRateLimiter(rateLimitConfig.stablecoinTransaction);

module.exports = { generalLimiter, stablecoinTransactionLimiter };
