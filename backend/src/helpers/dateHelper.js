// dateHelper.js
const moment = require('moment');

/**
 * Format a date into a readable string.
 * @param {Date|string|number} date - The date to format.
 * @param {string} [format='YYYY-MM-DD HH:mm:ss'] - (Optional) Format string.
 * @returns {string} - The formatted date string.
 */
const formatDate = (date, format = 'YYYY-MM-DD HH:mm:ss') => {
  return moment(date).format(format);
};

/**
 * Calculate the difference between two dates in a specified unit.
 * @param {Date|string|number} date1 - The first date.
 * @param {Date|string|number} date2 - The second date, defaults to now.
 * @param {string} [unit='days'] - (Optional) Unit of time to calculate difference in.
 * @returns {number} - The difference in the specified units.
 */
const dateDiff = (date1, date2 = new Date(), unit = 'days') => {
  return moment(date1).diff(moment(date2), unit);
};

module.exports = {
  formatDate,
  dateDiff,
};