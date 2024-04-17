// exchangeRateUpdater.js
const schedule = require('node-schedule');
const axios = require('axios');
const { updateExchangeRates } = require('../services/exchangeRateService');

/**
 * Fetches the latest exchange rates and updates the application's stored rates.
 */
async function fetchAndUpdateExchangeRates() {
  try {
    const { data } = await axios.get('https://api.exchangeratesapi.io/latest?base=USD');
    // Assume the API returns rates in the format { rates: { NEAR: 1.23, ... } }
    await updateExchangeRates(data.rates);
    console.log('Exchange rates updated successfully.');
  } catch (error) {
    console.error('Failed to update exchange rates:', error);
  }
}

/**
 * Schedule a task to update exchange rates every hour.
 */
function scheduleExchangeRateUpdates() {
  // This schedule is set to run at the start of every hour
  schedule.scheduleJob('0 * * * *', fetchAndUpdateExchangeRates);
}

module.exports = { scheduleExchangeRateUpdates };