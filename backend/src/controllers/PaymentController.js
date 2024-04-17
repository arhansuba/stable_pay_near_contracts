// src/controllers/PaymentController.js
const PaymentService = require('../services/paymentService/PaymentService');
const Payment = require('../models/payment');
const { validatePayment } = require('../utils/validationUtils');
const PaymentService = require('../services/paymentService/PaymentService');

exports.createPayment = async (req, res) => {
    // Validate request body against payment schema
    const { error, value } = validatePayment(req.body);
    if (error) {
        return res.status(400).json({ error: error.details[0].message });
    }

    try {
        // Process the payment through your service layer
        const paymentResult = await PaymentService.processPayment(value);
        res.json({ message: 'Payment processed successfully', paymentResult });
    } catch (err) {
        res.status(500).json({ error: 'Internal server error' });
    }
};
// Create a new payment transaction
exports.createPayment = async (req, res) => {
    const { senderAccountId, receiverAccountId, amount } = req.body;
    
    try {
        // Call the payment service to process the blockchain transaction
        const paymentResult = await PaymentService.sendPayment(senderAccountId, receiverAccountId, amount);
        
        // Optionally, save the payment details to your database
        const newPayment = new Payment({ senderAccountId, receiverAccountId, amount, status: 'pending' }); // Assume initial status is 'pending'
        await newPayment.save();
        
        res.status(201).json({ message: "Payment initiated successfully", paymentResult, newPayment });
    } catch (error) {
        console.error("Payment creation failed:", error);
        res.status(500).json({ error: "Failed to initiate payment" });
    }
};

// Get the status of a payment
exports.getPaymentStatus = async (req, res) => {
    const { paymentId } = req.params;
    
    try {
        const payment = await Payment.findById(paymentId);
        if (!payment) {
            return res.status(404).json({ message: "Payment not found" });
        }
        res.json(payment);
    } catch (error) {
        console.error("Error fetching payment status:", error);
        res.status(500).json({ error: "Failed to fetch payment status" });
    }
};

// List all payments for a user
exports.listPayments = async (req, res) => {
    const { accountId } = req.params;
    
    try {
        const payments = await Payment.find({ 
            $or: [{ senderAccountId: accountId }, { receiverAccountId: accountId }] 
        });
        res.json(payments);
    } catch (error) {
        console.error("Error listing payments:", error);
        res.status(500).json({ error: "Failed to list payments" });
    }
};