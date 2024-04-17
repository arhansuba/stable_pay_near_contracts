const Joi = require('joi');

// Validation schema for user registration
const userSchema = Joi.object({
    accountId: Joi.string().required(),
    name: Joi.string().min(3).max(30).optional(),
    email: Joi.string().email().optional(),
    // Add more fields as necessary
});

// Validation schema for payment request
const paymentSchema = Joi.object({
    senderAccountId: Joi.string().required(),
    receiverAccountId: Joi.string().required(),
    amount: Joi.number().positive().required(),
    currency: Joi.string().default('NEAR'),
    // Add more fields as necessary
});

// Function to validate a user object
const validateUser = (userObj) => {
    return userSchema.validate(userObj);
};

// Function to validate a payment object
const validatePayment = (paymentObj) => {
    return paymentSchema.validate(paymentObj);
};

// Exporting the utility functions
module.exports = {
    validateUser,
    validatePayment
};