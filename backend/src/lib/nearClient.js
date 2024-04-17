// nearClient.js
const { connect, WalletConnection, utils } = require('near-api-js');
const { networkId, nodeUrl, walletUrl, contractName } = require('../config/blockchainConfig');

class NearClient {
  constructor() {
    this.connection = null;
    this.wallet = null;
  }

  async initialize() {
    this.connection = await connect({
      networkId,
      nodeUrl,
      walletUrl,
      keyStore: new utils.keyStores.BrowserLocalStorageKeyStore(),
    });
    this.wallet = new WalletConnection(this.connection, contractName);
  }

  async signIn() {
    if (!this.wallet.isSignedIn()) {
      this.wallet.requestSignIn({
        contractId: contractName,
        methodNames: [], // Add method names if your contract requires specific access
      });
    }
  }

  signOut() {
    this.wallet.signOut();
    // Redirect or perform additional cleanup after sign-out
  }

  getAccountId() {
    return this.wallet.getAccountId();
  }

  // Additional helper methods for interacting with the blockchain can be implemented here
}

const nearClient = new NearClient();
module.exports = nearClient;