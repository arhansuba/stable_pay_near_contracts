// paymentProcessor.js
const nearClient = require('./nearClient');
const { utils } = require('near-api-js');

class PaymentProcessor {
  constructor() {
    this.nearClient = nearClient;
  }

  async sendPayment({ receiverId, amount }) {
    // Ensure initialization of NEAR client
    if (!this.nearClient.connection) {
      await this.nearClient.initialize();
    }

    const accountId = this.nearClient.getAccountId();
    if (!accountId) {
      throw new Error('User not signed in');
    }

    const amountInYoctoNear = utils.format.parseNearAmount(String(amount));
    const transaction = await this.nearClient.connection.account(accountId).sendMoney(
      receiverId, // receiver account
      amountInYoctoNear, // amount in yoctoNEAR
    );

    return transaction;
  }

  // Additional methods for handling different payment scenarios can be added here
}

const paymentProcessor = new PaymentProcessor();
module.exports = paymentProcessor;