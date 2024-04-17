const rateLimit = require('express-rate-limit');
const RedisStore = require('rate-limit-redis');
const { createClient } = require('redis');

// Assuming Redis is available and running
const redisClient = createClient({
  url: process.env.REDIS_URL, // Specify your Redis instance URL
  legacyMode: true,
});
redisClient.connect().catch(console.error);

const enhancedRateLimiter = rateLimit({
  store: new RedisStore({
    client: redisClient,
    expiry: 60 * 15, // Expiration in seconds (15 minutes)
  }),
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 200, // Limit each IP to 200 requests per windowMs
  standardHeaders: true, // Return rate limit info in the RateLimit-* headers
  legacyHeaders: false, // Disable the X-RateLimit-* headers
  handler: (req, res) => {
    res.status(429).json({
      error: "Too many requests, please try again later.",
    });
  },
});

module.exports = enhancedRateLimiter;