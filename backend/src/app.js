// Import required modules
const express = require('express');
const mongoose = require('mongoose');
require('dotenv').config();

// Import routes and middleware
const authService = require('./services/authService'); // Adjust path based on your project structure
const userRoutes = require('./routes/userRoutes');
const upgradeController = require('./upgradeController');
const multiSigWalletController = require('./multi_signature_wallet_controller');
const analyticsReportingController = require('./analytics_reporting_controller');
const upgradeRoutes = require('./upgrade_routes');
const analyticsReportingRoutes = require('./analytics_reporting_routes');
const errorHandler = require('./middleware/errorMiddleware');
const web3Middleware = require('./middleware/web3Middleware');
const { isFraudulentTransaction } = require('./fraud_detection_ml');
const { setupWebSocket } = require('./realtime/socketSetup');
const { basicRateLimiter, applyHelmet, configureCors, checkApiKey } = require('./securityEnhancements');

// Initialize Express app
const app = express();

// Use middleware
app.use(cors()); // Enable CORS for all requests
app.use(express.json()); // Parse JSON bodies
app.use(applyHelmet); // Apply Helmet for security headers
app.use(configureCors); // Set up CORS
app.use(basicRateLimiter); // Apply rate limiting to all routes

// Connect to MongoDB
mongoose.connect(process.env.MONGODB_URI, { useNewUrlParser: true, useUnifiedTopology: true })
    .then(() => console.log('Connected to MongoDB'))
    .catch(err => console.error('Could not connect to MongoDB:', err));

// Define routes
app.post('/api/auth', authService.authenticate); // Authentication route
app.use('/api/users', userRoutes); // User-related routes
app.use('/api/multi-sig-wallet', multiSigWalletController); // Multi-signature wallet routes
app.use('/api/analytics', analyticsReportingRoutes); // Analytics reporting routes
app.use('/api/upgrade', upgradeRoutes); // Upgrade routes

app.post('/process-transaction', async (req, res) => {
    const transactionData = req.body;
    try {
        const isFraud = await isFraudulentTransaction(transactionData);
        if (isFraud) {
            return res.status(400).json({ message: 'Transaction declined due to fraud suspicion.' });
        }
        // Proceed with transaction processing...
        res.status(200).json({ message: 'Transaction processed successfully.' });
    } catch (error) {
        res.status(500).json({ message: 'Error processing transaction.' });
    }
});

// Use the error handling middleware as the last piece of middleware
app.use(errorHandler);

// Set up WebSocket server

setupWebSocket(server);

const PORT = process.env.PORT || 3000;
server.listen(PORT, () => console.log(`Server running on port ${PORT}`));

const express = require("express");
const http = require("http");
const NotificationService = require("./notificationService");


const port = process.env.PORT || 3000;

// Initialize the NotificationService with the server
const notificationService = new NotificationService(server);

// Example use: sending a notification to all users
// This could be triggered by a specific event in your application
notificationService.broadcastNotification({
    title: "System Update",
    message: "System maintenance will occur at 12:00 AM UTC.",
});

server.listen(port, () => {
    console.log(`Server running on http://localhost:${port}`);
});

const express = require('express');
const logger = require('./logger');
const metricsMiddleware = require('./monitoringSetup');



app.use(metricsMiddleware);
app.use((req, res, next) => {
  logger.info(`${req.method} ${req.path}`);
  next();
});

// Your routes here


app.listen(PORT, () => logger.info(`Server running on http://localhost:${PORT}`));


const restAPIController = require('./RestAPIController');
app.use('/api', restAPIController);

const { createServer } = require('http');
const { initWebSocketServer } = require('./WebSocketController');

const server = createServer(app);

const { wss, broadcastNewPayment } = initWebSocketServer(server);

// Use `broadcastNewPayment` where appropriate in your services to push notifications to clients

server.listen(port, () => console.log(`Server started on port ${port}`));


app.use('/api', apiLimiter); // Apply general rate limiting
app.use('/api/login', loginLimiter); // Apply stricter rate limiting for login

const securityEnhancements = require('./config/SecurityEnhancements');
securityEnhancements(app); // Apply security configurations to the app

const { router: performanceRouter } = require('./PerformanceMonitoring');
app.use(performanceRouter);

const DecentralizedStorageService = require('./DecentralizedStorageService');
const storageService = new DecentralizedStorageService();

// Example usage
async function storeDocument(document) {
  const cid = await storageService.uploadToIPFS(document);
  console.log(`Document stored on IPFS with CID: ${cid}`);
}

