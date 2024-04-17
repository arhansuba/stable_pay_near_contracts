// src/middleware/errorMiddleware.js

// Error handling middleware have 4 parameters: (err, req, res, next)
const errorHandler = (err, req, res, next) => {
    // You can customize your error object depending on the structure
    // For demonstration, let's assume err has a statusCode and message property
    const statusCode = err.statusCode || 500; // Defaults to 500 if statusCode not specified
    const message = err.message || "An unexpected error occurred";

    console.error(err); // Log the error for debugging purposes

    // Send the error response
    res.status(statusCode).json({
        error: true,
        statusCode,
        message
    });
};

module.exports = errorHandler;