// src/routes/paymentRoutes.js
const express = require('express');
const router = express.Router();
const PaymentController = require('../controllers/PaymentController');
const paymentService = require('../services/paymentService')
// Route to initiate a new payment
router.post('/', PaymentController.createPayment);

// Route to get the status of a specific payment
router.get('/:paymentId/status', PaymentController.getPaymentStatus);

// Route to list all payments for a user
router.get('/user/:accountId', PaymentController.listPayments);

module.exports = router;