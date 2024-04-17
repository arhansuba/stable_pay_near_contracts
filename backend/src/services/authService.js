// services/authService.js

const { connect, keyStores } = require('near-api-js');
const { verifySignature } = require('../utils/cryptoUtils'); // You'll need to implement this

// Function to initialize a NEAR connection
async function initNear() {
  const config = {
    networkId: "testnet",
    keyStore: new keyStores.InMemoryKeyStore(),
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
  };

  const near = await connect(config);
  return near;
}

// Authentication function
exports.authenticate = async (req, res) => {
  const { accountId, publicKey, signature, data } = req.body;

  try {
    const near = await initNear();
    const account = await near.account(accountId);
    const accessKeys = await account.getAccessKeys();

    // Check if the provided public key is associated with the account
    if (!accessKeys.some(key => key.public_key === publicKey)) {
      return res.status(400).json({ error: "Public key does not belong to the account." });
    }

    // Verify the signature
    if (!verifySignature(data, signature, publicKey)) {
      return res.status(401).json({ error: "Signature verification failed." });
    }

    // Signature is valid; proceed with your login or token generation logic
    res.status(200).json({ message: "Authentication successful." });
  } catch (error) {
    console.error("Authentication error:", error);
    res.status(500).json({ error: "Internal server error." });
  }
};