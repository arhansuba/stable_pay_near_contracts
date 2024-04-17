// transactionCleanupScheduler.js
const schedule = require('node-schedule');
const { Transaction } = require('../models/Transaction');

/**
 * Schedule a daily cleanup task to remove old or completed transactions.
 */
function scheduleTransactionCleanup() {
  // This schedule is set to run every day at midnight
  schedule.scheduleJob('0 0 * * *', async function() {
    console.log('Running transaction cleanup task...');
    const cutoffDate = new Date();
    cutoffDate.setDate(cutoffDate.getDate() - 30); // Adjust based on your requirements

    try {
      const result = await Transaction.deleteMany({
        createdAt: { $lt: cutoffDate },
        status: 'completed', // Example status, adjust based on your application logic
      });
      console.log('Transaction cleanup completed:', result);
    } catch (error) {
      console.error('Transaction cleanup failed:', error);
    }
  });
}

module.exports = { scheduleTransactionCleanup };