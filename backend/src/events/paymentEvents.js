const customEventEmitter = require('./eventEmitter');
const { processPayment, notifyUser } = require('../services/paymentService');

// Listen for a 'paymentInitiated' event and process the payment
customEventEmitter.on('paymentInitiated', async (paymentDetails) => {
  try {
    const result = await processPayment(paymentDetails);
    customEventEmitter.emit('paymentCompleted', result);
  } catch (error) {
    customEventEmitter.emit('paymentFailed', { paymentDetails, error });
  }
});

// Listen for a 'paymentCompleted' event and notify the user
customEventEmitter.on('paymentCompleted', (paymentResult) => {
  notifyUser(paymentResult);
});

// Listen for a 'paymentFailed' event and handle the failure
customEventEmitter.on('paymentFailed', (data) => {
  console.error('Payment failed:', data.error);
  // Additional error handling logic here
});

module.exports = {
  // Export functions that emit events if necessary
};