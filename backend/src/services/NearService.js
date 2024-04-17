// NearService.js
const { connect, keyStores, WalletConnection } = require('near-api-js');
const config = require('./nearConfig'); // Assume you have a configuration file for NEAR

class NearService {
  constructor() {
    this.nearConfig = config;
    this.initConnection();
  }

  async initConnection() {
    this.near = await connect(this.nearConfig);
    this.wallet = new WalletConnection(this.near);
    this.contract = await new this.near.Contract(this.wallet.account(), config.contractName, {
      viewMethods: ['getContractData'], // Add your contract's view methods here
      changeMethods: ['createTransaction'], // Add your contract's change methods here
    });
  }

  async getContractData(parameter) {
    return this.contract.getContractData({ parameter });
  }

  async createTransaction(receiverId, amount) {
    // Assuming 'createTransaction' is a change method in your smart contract
    // Convert the amount to NEAR's smallest unit (yoctoNEAR) if necessary
    return this.contract.createTransaction({
      receiverId,
      amount: this.near.utils.format.parseNearAmount(amount.toString())
    });
  }
}

module.exports = NearService;