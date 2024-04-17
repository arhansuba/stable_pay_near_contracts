// blockchainConfig.js
const { connect, keyStores, WalletConnection } = require('near-api-js');
const path = require('path');
require('dotenv').config();
const usdcMainnetContract = process.env.USDC_MAINNET_CONTRACT;
const usdcTestnetContract = process.env.USDC_TESTNET_CONTRACT;
const { connect, Contract } = require('near-api-js');
const config = require('./config'); // Your NEAR configuration

async function getUSDCBalance(accountId, isTestnet = true) {
    const near = await connect(config);
    const account = await near.account(accountId);
    const contractAddress = isTestnet ? usdcTestnetContract : usdcMainnetContract;
    const contract = new Contract(account, contractAddress, {
        viewMethods: ['ft_balance_of'],
        changeMethods: [],
    });

    const balance = await contract.ft_balance_of({ account_id: accountId });
    return balance;
}


const environment = process.env.NODE_ENV || 'development';
const isDevelopment = environment === 'development';

const networkConfig = {
  mainnet: {
    networkId: 'mainnet',
    nodeUrl: 'https://rpc.mainnet.near.org',
    walletUrl: 'https://wallet.near.org',
    helperUrl: 'https://helper.mainnet.near.org',
  },
  testnet: {
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    walletUrl: 'https://wallet.testnet.near.org',
    helperUrl: 'https://helper.testnet.near.org',
  }
};

const config = isDevelopment ? networkConfig.testnet : networkConfig.mainnet;

async function initializeConnection() {
  // Setting up the keyStore for signing transactions. This example uses an unencrypted filesystem keystore for simplicity.
  // Ensure you use a more secure keystore for production!
  const keyStore = new keyStores.UnencryptedFileSystemKeyStore(path.join(__dirname, './near-credentials'));

  // Connecting to NEAR
  const near = await connect({
    deps: {
      keyStore,
    },
    ...config,
  });

  // Creating a wallet connection
  const wallet = new WalletConnection(near);

  return { near, wallet };
}

module.exports = { initializeConnection };