// nearConfig.js
const { keyStores } = require('near-api-js');
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

module.exports = {
  networkId: 'testnet',
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  contractName: 'decentralized_venmo_contracts.testnet',
  keyStore: new keyStores.InMemoryKeyStore(), // Consider using a more secure KeyStore in production
  // Other necessary config...
};

require('dotenv').config();
const usdcMainnetContract = process.env.USDC_MAINNET_CONTRACT;
const usdcTestnetContract = process.env.USDC_TESTNET_CONTRACT;


const { connect } = nearAPI;

const connectionConfig = {
  networkId: "testnet",
  keyStore: myKeyStore, // first create a key store
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://testnet.mynearwallet.com/",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://testnet.nearblocks.io",
};
const nearConnection = await connect(connectionConfig);