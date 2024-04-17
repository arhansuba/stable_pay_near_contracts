const { connect, WalletConnection, Contract, KeyPair, utils } = require('near-api-js');
const nearConfig = require('../config/nearConfig');

// Function to initialize a NEAR connection
async function initNear() {
  const near = await connect({
    ...nearConfig,
    deps: { keyStore: new nearAPI.keyStores.InMemoryKeyStore() },
  });

  const walletConnection = new WalletConnection(near);
  return { near, walletConnection };
}

// Example function to call a smart contract method
exports.callContractMethod = async (methodName, args) => {
  const { walletConnection } = await initNear();
  const account = walletConnection.account();

  const contract = new Contract(account, nearConfig.contractName, {
    // View methods are read-only – they don't modify the state
    viewMethods: [],
    // Change methods can modify the state
    changeMethods: [methodName],
  });

  try {
    const result = await contract[methodName](args, nearConfig.GAS, nearConfig.attachedDeposit);
    return result;
  } catch (error) {
    console.error(`Error calling contract method ${methodName}:`, error);
    throw error;
  }
};

// Example function to send NEAR tokens
exports.sendNearTokens = async (receiverId, amountInNear) => {
  const { walletConnection } = await initNear();
  const account = walletConnection.account();

  try {
    const amountInYoctoNear = utils.format.parseNearAmount(amountInNear.toString());
    const result = await account.sendMoney(receiverId, amountInYoctoNear);
    return result;
  } catch (error) {
    console.error('Error sending NEAR tokens:', error);
    throw error;
  }
};