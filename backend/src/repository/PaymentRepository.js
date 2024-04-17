const mongoose = require('mongoose');

class PaymentRepository {
    constructor(model) {
        this.model = model; // Assume this is a Mongoose model for Payments
    }

    // Create a payment with transaction support
    async createPayment(paymentData, session = null) {
        try {
            const payment = new this.model(paymentData);
            await payment.save({ session });
            return payment;
        } catch (error) {
            console.error('Error creating payment:', error);
            throw new Error('Payment creation failed');
        }
    }

    // Fetch a payment by ID with error handling
    async getPaymentById(id) {
        try {
            const payment = await this.model.findById(id);
            if (!payment) {
                throw new Error('Payment not found');
            }
            return payment;
        } catch (error) {
            console.error('Error fetching payment by ID:', error);
            throw error;
        }
    }

    // Update a payment status
    async updatePaymentStatus(id, newStatus, session = null) {
        try {
            const result = await this.model.updateOne({ _id: id }, { status: newStatus }, { session });
            if (result.matchedCount === 0) {
                throw new Error('Payment not found');
            }
            return result;
        } catch (error) {
            console.error('Error updating payment status:', error);
            throw error;
        }
    }

    // Aggregate payments based on status for analytics
    async aggregatePaymentsByStatus() {
        try {
            return await this.model.aggregate([
                { $group: { _id: "$status", count: { $sum: 1 } } },
                { $sort: { count: -1 } }
            ]);
        } catch (error) {
            console.error('Error aggregating payments by status:', error);
            throw error;
        }
    }

    // Implement bulk payment creation with transaction support
    async createBulkPayments(paymentsData) {
        const session = await mongoose.startSession();
        session.startTransaction();
        try {
            const payments = await Promise.all(paymentsData.map(async (paymentData) => 
                this.createPayment(paymentData, session)
            ));
            await session.commitTransaction();
            return payments;
        } catch (error) {
            await session.abortTransaction();
            console.error('Error creating bulk payments:', error);
            throw error;
        } finally {
            session.endSession();
        }
    }

    
}

module.exports = PaymentRepository;