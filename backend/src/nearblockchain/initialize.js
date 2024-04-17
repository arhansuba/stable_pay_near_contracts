// In your backend setup file or a dedicated NEAR module
const { connect, keyStores, WalletConnection } = require('near-api-js');
const express = require('express');
const app = express();

async function initNear() {
    const near = await connect({
        networkId: "testnet",
        keyStore: new keyStores.InMemoryKeyStore(),
        nodeUrl: "https://rpc.testnet.near.org",
        walletUrl: "https://wallet.testnet.near.org",
    });
    return near;
}

app.get('/fetch-data', async (req, res) => {
    const near = await initNear();
    // Use the `near` instance to interact with your smart contract
});

app.listen(3000, () => console.log(`Backend running on http://localhost:3000`));