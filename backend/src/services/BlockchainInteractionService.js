// BlockchainInteractionService.js

const { connect, Contract, keyStores, WalletConnection } = require('near-api-js');
const config = require('../config/nearConfig');

class BlockchainInteractionService {
  constructor() {
    this.nearConfig = config;
    this.contract = null;
    this.wallet = null;
  }

  async init() {
    try {
      const near = await connect(this.nearConfig);
      const walletConnection = new WalletConnection(near, null);
      const walletAccount = walletConnection.account();
      this.contract = new Contract(
        walletAccount,
        this.nearConfig.stablecoinContract, // Assumed stablecoin contract name in config
        {
          viewMethods: ['ft_balance_of', 'ft_total_supply'],
          changeMethods: ['ft_transfer', 'ft_transfer_call'],
        }
      );
      this.wallet = walletConnection;
    } catch (error) {
      console.error('Error initializing blockchain interaction service:', error);
      throw error; // Rethrow the error for handling in higher layers
    }
  }

  async getStablecoinBalance(accountId) {
    try {
      await this.init(); // Ensure contract is initialized
      return await this.contract.ft_balance_of({ account_id: accountId });
    } catch (error) {
      console.error('Error getting stablecoin balance:', error);
      throw error; // Rethrow the error for handling in higher layers
    }
  }

  async transferStablecoins({ senderId, receiverId, amount }) {
    try {
      await this.init(); // Ensure contract is initialized
      // Note: Actual transfer may require the sender to sign the transaction, which can't be done directly in the backend without the sender's private key.
      // Consider implementing a method that creates a transaction for the frontend to sign.
      const options = {
        // Set gas and attached deposit values
        gas: 30000000000000,
        deposit: 1,
      };
      const result = await this.contract.ft_transfer(
        {
          receiver_id: receiverId,
          amount: amount.toString(),
          memo: 'Stablecoin transfer',
        },
        options
      );
      return result;
    } catch (error) {
      console.error('Error transferring stablecoins:', error);
      throw error; // Rethrow the error for handling in higher layers
    }
  }

  async getAccountNonce(accountId) {
    try {
      await this.init(); // Ensure contract is initialized
      const nonce = await this.wallet.account().getAccountDetails(accountId).nonce;
      return nonce;
    } catch (error) {
      console.error('Error getting account nonce:', error);
      throw error; // Rethrow the error for handling in higher layers
    }
  }
  // BlockchainInteractionService.js
async sendPayment({ senderId, receiverId, amount }) {
  // Logic to call the smart contract's send_payment method
}

async getBalance(accountId) {
  // Logic to call the get_balance method
}
}

module.exports = new BlockchainInteractionService();
