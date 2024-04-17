// responseHelper.js

/**
 * Send a success response.
 * @param {Object} res - The response object.
 * @param {Object} data - The response data.
 * @param {number} [statusCode=200] - (Optional) HTTP status code.
 */
const sendSuccess = (res, data, statusCode = 200) => {
    res.status(statusCode).json({
      success: true,
      data,
    });
  };
  
  /**
   * Send an error response.
   * @param {Object} res - The response object.
   * @param {string} message - Error message.
   * @param {number} [statusCode=500] - (Optional) HTTP status code.
   */
  const sendError = (res, message, statusCode = 500) => {
    res.status(statusCode).json({
      success: false,
      error: message,
    });
  };
  
  /**
   * Handle not found resources.
   * @param {Object} res - The response object.
   * @param {string} [message='Resource not found.'] - (Optional) Custom error message.
   */
  const notFound = (res, message = 'Resource not found.') => sendError(res, message, 404);
  
  module.exports = {
    sendSuccess,
    sendError,
    notFound,
  };