// PerformanceMonitoring.js
const client = require('prom-client');
const express = require('express');
const router = express.Router();

// Create a Registry to register the metrics
const register = new client.Registry();

// Enable the collection of default metrics
client.collectDefaultMetrics({ register });

// Custom blockchain interaction metrics
const blockchainRequests = new client.Counter({
  name: 'blockchain_requests_total',
  help: 'Total number of blockchain requests made',
  registers: [register],
});

// Function to increment blockchain interaction counter
function incrementBlockchainRequests() {
  blockchainRequests.inc(); // Increment by 1
}

// Expose the metrics at '/metrics' endpoint
router.get('/metrics', async (req, res) => {
  try {
    res.set('Content-Type', register.contentType);
    res.end(await register.metrics());
  } catch (err) {
    res.status(500).end(err);
  }
});

module.exports = { router, incrementBlockchainRequests };