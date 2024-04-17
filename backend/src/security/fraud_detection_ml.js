// fraud_detection_ml.js
const axios = require('axios');

const ML_SERVICE_URL = 'http://your-ml-model-service.com/predict';

/**
 * Simulates sending transaction data to an ML service for fraud prediction.
 * @param {Object} transactionData The transaction data to be evaluated.
 * @returns {Promise<Boolean>} A promise that resolves to true if fraud is detected, otherwise false.
 */
async function isFraudulentTransaction(transactionData) {
    try {
        // Send transaction data to ML service for prediction
        const response = await axios.post(ML_SERVICE_URL, transactionData);

        // Assume the ML service returns a JSON object with a prediction field
        const { prediction } = response.data;
        
        // Interpret the prediction (true for fraudulent, false for non-fraudulent)
        return prediction === 'fraudulent';
    } catch (error) {
        console.error('Error calling ML service:', error);
        // In a real application, you might want to handle this more gracefully
        throw new Error('Failed to get fraud prediction from ML service.');
    }
}

module.exports = { isFraudulentTransaction };