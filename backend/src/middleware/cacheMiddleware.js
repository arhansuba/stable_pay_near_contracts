const { createClient } = require('redis');
const redisClient = createClient({
  url: process.env.REDIS_URL, // Your Redis URL
});
redisClient.connect().catch(console.error);

const cacheMiddleware = (expiryDuration = 60) => async (req, res, next) => {
    if (req.method !== 'GET') {
        console.log('Bypass cache for non-GET request');
        return next();
    }

    const cacheKey = `cache:${req.originalUrl}`;
    try {
        const cachedResponse = await redisClient.get(cacheKey);
        if (cachedResponse) {
            console.log(`Serving from cache: ${req.originalUrl}`);
            return res.send(JSON.parse(cachedResponse));
        } else {
            console.log(`Cache miss: ${req.originalUrl}`);
            const originalSend = res.send.bind(res);
            res.send = async (body) => {
                try {
                    await redisClient.setEx(cacheKey, expiryDuration, body);
                    console.log(`Response cached: ${req.originalUrl}`);
                } catch (error) {
                    console.error('Error caching response:', error);
                }
                originalSend(body);
            };
            next();
        }
    } catch (error) {
        console.error('Cache middleware error:', error);
        next(); // Proceed without caching in case of error
    }
};

module.exports = cacheMiddleware;