// Import necessary modules from near-api-js
const { connect, keyStores, WalletConnection } = require('near-api-js');

// This is a simple configuration. Ensure you replace it with your actual configuration.
const nearConfig = {
  networkId: "testnet",
  keyStore: new keyStores.InMemoryKeyStore(), // This is a simple key store, consider using a more secure key store for production
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://wallet.testnet.near.org",
  helperUrl: "https://helper.testnet.near.org",
};

// Middleware function to initialize NEAR connection
async function web3Middleware(req, res, next) {
  try {
    // Connect to NEAR
    const near = await connect(nearConfig);
    const wallet = new WalletConnection(near);

    // Attach `near` and `wallet` to the request object
    req.near = near;
    req.wallet = wallet;

    next(); // Pass control to the next handler
  } catch (error) {
    console.error("Failed to initialize NEAR connection:", error);
    res.status(500).send("Failed to initialize blockchain connection.");
  }
}

module.exports = web3Middleware;