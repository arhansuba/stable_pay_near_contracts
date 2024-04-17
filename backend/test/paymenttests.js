const request = require('supertest');
const app = require('../src/app'); // Adjust the path

describe('Payment Processing', () => {
  it('should create a new payment', async () => {
    // Assuming the user needs to be authenticated to create a payment
    // You'd first log in or use a mock authentication middleware for testing
    const res = await request(app)
      .post('/api/payments')
      .send({
        senderAccountId: 'sender.testnet',
        receiverAccountId: 'receiver.testnet',
        amount: 100
      });
    expect(res.statusCode).toEqual(201);
    expect(res.body).toHaveProperty('message');
  });
});